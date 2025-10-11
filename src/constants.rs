pub mod sensor {
    pub const GPU_LABEL: &str = "amdgpu";
    pub const NVME_LABEL: &str = "nvme Composite";
    pub const CPU_TEMP_LABEL: &str = "Tctl";
}

pub mod metrics {
    pub const TYPE_USAGE: &str = "usage";
    pub const TYPE_TEMPERATURE: &str = "temperature";

    pub const NAME_CPU_USAGE: &str = "cpu_usage";
    pub const NAME_MEMORY_USAGE: &str = "memory_usage";
    pub const NAME_GPU_TEMPERATURE: &str = "gpu_temperature";
    pub const NAME_NVME_TEMPERATURE: &str = "nvme_temperature";
    pub const NAME_CPU_TEMPERATURE: &str = "cpu_temperature";
}

pub mod units {
    pub const PERCENT: &str = "percent";
    pub const CELSIUS: &str = "celsius";
}
