use std::env;

pub struct Config {
    pub telemetry: bool,
}

impl Config {
    pub fn build() -> Self {
        Config {
            telemetry: env::var("OTEL_SERVICE_NAME").is_ok(),
        }
    }
}
