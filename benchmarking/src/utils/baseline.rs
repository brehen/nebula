use super::modbus::SensorData;

pub fn measure_baseline(sensor_data: Vec<SensorData>) -> f32 {
    let mut total_power = 0.0;
    let total = sensor_data.len();
    for reading in sensor_data {
        total_power += reading.power;
    }
    if total_power > 0.0 {
        total_power / total as f32
    } else {
        0.0
    }
}
