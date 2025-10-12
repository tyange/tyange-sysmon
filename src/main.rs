use std::env;

use chrono::{DateTime, Utc};
use dotenv::dotenv;
use postgres::{NoTls, Transaction};
use sysinfo::{Components, System};

use crate::constants::{metrics, sensor, units};

mod constants;

fn round_to_decimals(value: f64, decimals: u32) -> f64 {
    let multiplier = 10_f64.powi(decimals as i32);
    (value * multiplier).round() / multiplier
}

fn insert_cpu_usage(
    sys: &System,
    trans: &mut Transaction<'_>,
    now: DateTime<Utc>,
) -> Result<u64, postgres::Error> {
    let cpu_usage = round_to_decimals(sys.global_cpu_usage() as f64, 2);

    trans.execute(
        "INSERT INTO system_metrics (time, metric_type, metric_name, value, unit) VALUES ($1, $2, $3, $4, $5)",
        &[&now, &metrics::TYPE_USAGE, &metrics::NAME_CPU_USAGE, &cpu_usage, &units::PERCENT],
    )
}

fn insert_memory_usage(
    sys: &System,
    trans: &mut Transaction<'_>,
    now: DateTime<Utc>,
) -> Result<u64, postgres::Error> {
    let total_memory = sys.total_memory() as f64;
    let used_memory = sys.used_memory() as f64;
    let memory_usage = round_to_decimals((used_memory / total_memory) * 100.0, 2);

    trans.execute(
        "INSERT INTO system_metrics (time, metric_type, metric_name, value, unit) VALUES ($1, $2, $3, $4, $5)",
        &[&now, &metrics::TYPE_USAGE, &metrics::NAME_MEMORY_USAGE, &memory_usage, &units::PERCENT],
    )
}

fn insert_components_temp(
    comps: &Components,
    trans: &mut Transaction<'_>,
    now: DateTime<Utc>,
) -> Result<(), postgres::Error> {
    comps
        .iter()
        .filter_map(|comp| comp.temperature().map(|temp| (comp.label(), temp as f64)))
        .try_for_each(|(name, temp)| -> Result<(), postgres::Error> {
            let rounded_temp = round_to_decimals(temp, 2);

            if name.contains(sensor::GPU_LABEL) {
                 trans.execute(
                    "INSERT INTO system_metrics (time, metric_type, metric_name, value, unit) VALUES ($1, $2, $3, $4, $5)",
                    &[&now, &metrics::TYPE_TEMPERATURE, &metrics::NAME_GPU_TEMPERATURE, &rounded_temp, &units::CELSIUS],
                )?;
            } else if name.contains(sensor::NVME_LABEL) {
                 trans.execute(
                    "INSERT INTO system_metrics (time, metric_type, metric_name, value, unit) VALUES ($1, $2, $3, $4, $5)",
                    &[&now, &metrics::TYPE_TEMPERATURE, &metrics::NAME_NVME_TEMPERATURE, &rounded_temp, &units::CELSIUS],
                )?;
            } else if name.contains(sensor::CPU_TEMP_LABEL) {
                 trans.execute(
                    "INSERT INTO system_metrics (time, metric_type, metric_name, value, unit) VALUES ($1, $2, $3, $4, $5)",
                    &[&now, &metrics::TYPE_TEMPERATURE, &metrics::NAME_CPU_TEMPERATURE, &rounded_temp, &units::CELSIUS],
                )?;
            } else {
                trans.execute(
                    "INSERT INTO system_metrics (time, metric_type, metric_name, value, unit) VALUES ($1, $2, $3, $4, $5)",
                    &[&now, &metrics::TYPE_TEMPERATURE, &name, &rounded_temp, &units::CELSIUS],
                )?;
            }

            Ok(())
        })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let host = env::var("DB_HOST")?;
    let port = env::var("DB_PORT")?;
    let user = env::var("DB_USER")?;
    let password = env::var("DB_PASSWORD")?;
    let dbname = env::var("DB_NAME")?;

    let mut client = postgres::Client::connect(
        &format!(
            "host={} port={} user={} password={} dbname={}",
            host, port, user, password, dbname,
        ),
        NoTls,
    )
    .unwrap();

    let now = Utc::now();
    let sys = System::new_all();
    let comps = Components::new_with_refreshed_list();

    let mut trans = client.transaction().unwrap();

    insert_cpu_usage(&sys, &mut trans, now)?;
    println!("Success cpu usage insert.");

    insert_memory_usage(&sys, &mut trans, now)?;
    println!("Success memory usage insert.");

    insert_components_temp(&comps, &mut trans, now)?;
    println!("Success temperature insert.");

    trans.commit()?;
    println!("Transaction committed successfully.");

    Ok(())
}
