use serde_derive::{Deserialize, Serialize};
use std::{
    io::Error,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio_modbus::prelude::*;

pub async fn read_modbus_data(addr: &str) -> Result<SensorData, anyhow::Error> {
    let mut current_client = tcp::connect(addr.parse()?).await?;
    let mut power_factor_client = tcp::connect(addr.parse()?).await?;

    let mut sensor_data = SensorData {
        current: 0.0,
        voltage: 240,
        power_factor: 0.0, // Assuming this is a floating-point value
        power: 0.0,
        start_read: current_micros()?,
        end_read: 0,
    };

    let current_task = tokio::spawn(async move {
        current_client
            .read_input_registers(LineInEnergySensor::Current as u16, 2)
            .await
            .map(|regs| (i32::from(regs[0]) << 16) | i32::from(regs[1]))
    });

    let power_factor_task = tokio::spawn(async move {
        power_factor_client
            .read_input_registers(LineInEnergySensor::PowerFactor as u16, 2)
            .await
            .map(|regs| (i32::from(regs[0]) << 16) | i32::from(regs[1]))
    });

    let raw_current = current_task.await??;
    let raw_power = power_factor_task.await??;

    sensor_data.current = (raw_current as f32) / 1000.0;
    sensor_data.power_factor = (raw_power as f32) / 1000.0;

    sensor_data.power = get_power(&sensor_data);
    sensor_data.end_read = current_micros()?;
    Ok(sensor_data)
}

pub fn print_readings(sensor_data: &SensorData) {
    println!(
        "{} ms since epoch, Readings: ({}V / {}mA / {}PF) = {:.2}W, Time taken: {} ms",
        sensor_data.start_read,
        sensor_data.voltage,
        sensor_data.current,
        sensor_data.power_factor,
        (240.0 * sensor_data.current * sensor_data.power_factor),
        sensor_data.end_read - sensor_data.start_read
    );
}

fn get_power(sensor_data: &SensorData) -> f32 {
    (sensor_data.voltage as f32) * sensor_data.current * sensor_data.power_factor
}

pub fn current_micros() -> std::io::Result<u128> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_micros())
        .map_err(|err| Error::new(std::io::ErrorKind::Other, err))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorData {
    pub current: f32,
    pub voltage: u32,
    pub power_factor: f32, // Assuming this is a floating-point value
    pub power: f32,
    pub start_read: u128,
    pub end_read: u128,
}

#[derive(Debug, Clone, Copy)]
pub enum LineInEnergySensor {
    Current = 0x406,
    PowerFactor = 0x40a,
}

impl LineInEnergySensor {
    pub fn address(self) -> u16 {
        self as u16
    }
}
