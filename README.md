# tyange-sysmon

tyange-homeserver의 status를 수집하는 Rust 언어로 만들어진 프로그램입니다. sysinfo라는 크레이트를 이용해 stat을 조회하고 TimescaleDB라는 PostgreSQL을 확장한 오픈 소스 시계열 데이터베이스에 저장합니다. tyange-homeserver에서 5분마다 한 번씩 실행되고 있습니다.

with [sysinfo](https://docs.rs/sysinfo/latest/sysinfo) & [TimescaleDB](https://github.com/timescale/timescaledb)