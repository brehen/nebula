use serde_derive::Serialize;

pub mod baseline;
pub mod calc;
pub mod efficiency_statistics;
pub mod energy_statistics;
pub mod file;
pub mod modbus;
pub mod mqtt;
pub mod power_estimate;
pub mod request;

#[derive(Default, Debug, Clone, Copy, Serialize)]
pub struct Metrics {
    pub min: f64,
    pub max: f64,
    pub median: f64,
    pub mean: f64,          // New field
    pub std_deviation: f64, // New field
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            min: f64::MAX,
            max: f64::MIN,
            median: 0.0,
            mean: 0.0,
            std_deviation: 0.0,
        }
    }
}
