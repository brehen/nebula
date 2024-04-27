use std::time::SystemTime;
use tokio_modbus::prelude::*;

const ADDR: &str = "192.168.68.66:502";

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let addr = ADDR.parse().unwrap();

    // let mut range = 0;

    rt.block_on(async {
        loop {
            // if range > 5 {
            //     break;
            // }
            //
            // range += 1;
            // Establish a new connection for each read
            if let Ok(mut client) = tcp::connect(addr).await {
                let start_time = SystemTime::now();
                // Read from register addresses 1028 and 1029 (for 32-bit voltage value)
                //
                let mut sensor_data = init_sensor_data();

                for sensor_type in SENSOR_ARRAY {
                    match client.read_input_registers(sensor_type.address(), 2).await {
                        Ok(registers) => match sensor_type {
                            LineInEnergySensor::Current => {
                                let value =
                                    (i32::from(registers[0]) << 16) | i32::from(registers[1]);
                                sensor_data.current = value;
                            }
                            LineInEnergySensor::PowerFactor => {
                                let raw_value =
                                    (i32::from(registers[0]) << 16) | i32::from(registers[1]);
                                let value = raw_value as f32 / 1000.0;
                                sensor_data.power_factor = value
                            }
                            _ => println!("How did we get here"),
                        },
                        Err(e) => println!("Error reading registers: {:?}", e),
                    }
                }

                let elapsed = start_time.elapsed().unwrap().as_micros();
                println!(
                    "{} ms since epoch, Readings: ({}V / {}mA / {}PF) = {:.2}W, Time taken: {} ms",
                    start_time
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_millis(),
                    240,
                    sensor_data.current,
                    sensor_data.power_factor,
                    (240.0 * (sensor_data.current as f32 / 1000.0) * sensor_data.power_factor),
                    elapsed
                );
            } else {
                println!("Failed to connect to Modbus server");
            }
            // time::sleep(Duration::from_millis(10)).await; // Adjust the delay as necessary
        }
    });
}

fn init_sensor_data() -> SensorData {
    SensorData {
        absolute_active_energy: 0,
        power_active: 0,
        voltage: 0,
        current: 0,
        frequency: 0,
        power_factor: 0.0, // Assuming this is a floating-point value
        power_apparent: 0,
        power_reactive: 0,
        absolute_active_energy_resettable: 0,
        absolute_reactive_energy: 0,
        absolute_reactive_energy_resettable: 0,
        forward_active_energy: 0,
        forward_reactive_energy: 0,
        forward_active_energy_resettable: 0,
        forward_reactive_energy_resettable: 0,
        reverse_active_energy: 0,
        reverse_reactive_energy: 0,
        reverse_active_energy_resettable: 0,
        reverse_reactive_energy_resettable: 0,
        residual_current_type_a: 0,
        neutral_current: 0,
    }
}

#[derive(Debug)]
pub struct SensorData {
    pub absolute_active_energy: i32,
    pub power_active: i32,
    pub voltage: i32,
    pub current: i32,
    pub frequency: i32,
    pub power_factor: f32, // Assuming this is a floating-point value
    pub power_apparent: i32,
    pub power_reactive: i32,
    pub absolute_active_energy_resettable: i32,
    pub absolute_reactive_energy: i32,
    pub absolute_reactive_energy_resettable: i32,
    // pub reset_time_seconds_since_reset: u32, // Assuming this is an unsigned 32-bit value
    pub forward_active_energy: i32,
    pub forward_reactive_energy: i32,
    pub forward_active_energy_resettable: i32,
    pub forward_reactive_energy_resettable: i32,
    pub reverse_active_energy: i32,
    pub reverse_reactive_energy: i32,
    pub reverse_active_energy_resettable: i32,
    pub reverse_reactive_energy_resettable: i32,
    pub residual_current_type_a: i32,
    pub neutral_current: i32,
}

const SENSOR_ARRAY: [LineInEnergySensor; 2] = [
    // LineInEnergySensor::AbsoluteActiveEnergy,
    // LineInEnergySensor::PowerApparent,
    // LineInEnergySensor::Voltage,
    LineInEnergySensor::Current,
    // LineInEnergySensor::Frequency,
    LineInEnergySensor::PowerFactor,
    // LineInEnergySensor::PowerReactive,
    // LineInEnergySensor::ForwardActiveEnergy,
    // LineInEnergySensor::ForwardReactiveEnergy,
    // LineInEnergySensor::ForwardActiveEnergyResettable,
    // LineInEnergySensor::ForwardReactiveEnergyResettable,
    // LineInEnergySensor::ReverseActiveEnergy,
    // LineInEnergySensor::ReverseReactiveEnergy,
    // LineInEnergySensor::ReverseActiveEnergyResettable,
    // LineInEnergySensor::ReverseReactiveEnergyResettable,
    // LineInEnergySensor::ResidualCurrentTypeA,
    // LineInEnergySensor::NeutralCurrent,
];

#[derive(Debug, Clone, Copy)]
pub enum LineInEnergySensor {
    AbsoluteActiveEnergy = 0x400,
    PowerActive = 0x402,
    Voltage = 0x404,
    Current = 0x406,
    Frequency = 0x408,
    PowerFactor = 0x40a,
    // PowerAngle = 0x40c,
    PowerApparent = 0x40e,
    PowerReactive = 0x410,
    AbsoluteActiveEnergyResettable = 0x412,
    AbsoluteReactiveEnergy = 0x414,
    AbsoluteReactiveEnergyResettable = 0x416,
    // ResetTimeSecSinceReset = 0x418,
    ForwardActiveEnergy = 0x41a,
    ForwardReactiveEnergy = 0x41c,
    ForwardActiveEnergyResettable = 0x41e,
    ForwardReactiveEnergyResettable = 0x420,
    ReverseActiveEnergy = 0x422,
    ReverseReactiveEnergy = 0x424,
    ReverseActiveEnergyResettable = 0x426,
    ReverseReactiveEnergyResettable = 0x428,
    ResidualCurrentTypeA = 0x42a,
    NeutralCurrent = 0x42c,
}

impl LineInEnergySensor {
    pub fn address(self) -> u16 {
        self as u16
    }
}

// match client
//     .read_input_registers(LineInEnergySensor::PowerApparent.address(), 2)
//     .await
// {
//     Ok(registers) if registers.len() == 2 => {
//         let voltage = (i32::from(registers[0]) << 16) | i32::from(registers[1]);
//         // let voltage_scaled = voltage as f32 / 100.0; // Assuming scaling factor of 0.01 V per unit
//         let elapsed = start_time.elapsed().unwrap().as_micros();
//         println!(
//             "{} ms since epoch, Power: {} W, Time taken: {} ms",
//             start_time
//                 .duration_since(SystemTime::UNIX_EPOCH)
//                 .unwrap()
//                 .as_millis(),
//             voltage,
//             elapsed
//         );
//     }
//     Ok(_) => println!("Error: Not enough registers read"),
//     Err(e) => println!("Error reading registers: {:?}", e),
// }
//

// // use std::time::SystemTime;
// // use tokio::{
// //     runtime::Runtime,
// //     time::{self, Duration},
// // };
// // use tokio_modbus::prelude::*;
// //
// // async fn read_voltage(addr: std::net::SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
// //     // Establish a new connection each time
// //     let mut client = tcp::connect(addr).await?;
// //     let start_time = SystemTime::now();
// //
// //     // Read from register addresses 1028 and 1029 (for 32-bit voltage value)
// //     let registers = client.read_input_registers(1028, 2).await?;
// //
// //     if registers.len() == 2 {
// //         let voltage = (i32::from(registers[0]) << 16) | i32::from(registers[1]);
// //         let voltage_scaled = voltage as f32 / 100.0; // Assuming scaling factor of 0.01 V per unit
// //         let elapsed = start_time.elapsed()?.as_millis();
// //         println!(
// //             "{} ms since epoch, Voltage: {} V, Time taken: {} ms",
// //             start_time
// //                 .duration_since(SystemTime::UNIX_EPOCH)?
// //                 .as_millis(),
// //             voltage_scaled,
// //             elapsed
// //         );
// //     } else {
// //         println!("Error: Not enough registers read");
// //     }
// //
// //     Ok(())
// // }
// //
// // async fn continuous_voltage_read(
// //     addr: std::net::SocketAddr,
// // ) -> Result<(), Box<dyn std::error::Error>> {
// //     loop {
// //         read_voltage(addr).await?;
// //         time::sleep(Duration::from_millis(50)).await; // Adjust the delay as necessary
// //     }
// // }
// //
// // fn main() {
// //     let rt = Runtime::new().unwrap();
// //     let addr = "192.168.68.53:502".parse().unwrap();
// //
// //     rt.spawn(async move {
// //         continuous_voltage_read(addr)
// //             .await
// //             .expect("Failed to read voltage continuously");
// //     });
// //
// //     loop {
// //         std::thread::sleep(Duration::from_secs(10));
// //     }
// // }
//
// use std::time::SystemTime;
// use tokio::{
//     runtime::Runtime,
//     time::{self, Duration},
// };
// use tokio_modbus::{client::Context, prelude::*};
//
// async fn read_voltage(mut client: Context) -> Result<(), Box<dyn std::error::Error>> {
//     // Address 0x404 for voltage, converting to Modbus index 0x404 (1028 decimal)
//     let start_time = SystemTime::now();
//
//     println!("70");
//
//     let registers = client.read_input_registers(1028, 2).await?;
//
//     println!("74");
//
//     if registers.len() == 2 {
//         println!("75");
//         let voltage = (i32::from(registers[0]) << 16) | i32::from(registers[1]); // Combine two registers
//         let voltage_scaled = voltage as f32 / 100.0; // Assuming voltage scaling factor of 0.01 V
//         let elapsed = start_time.elapsed()?.as_millis();
//         println!(
//             "{} ms since epoch, Voltage: {} V, Time taken: {} ms",
//             start_time
//                 .duration_since(SystemTime::UNIX_EPOCH)?
//                 .as_millis(),
//             voltage_scaled,
//             elapsed
//         );
//     } else {
//         println!("Error: Not enough registers read");
//     }
//
//     Ok(())
// }
//
// async fn continuous_voltage_read(
//     addr: std::net::SocketAddr,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     println!("Reading");
//     let transport = tokio::net::TcpStream::connect(addr).await?;
//     let ctx = tokio_modbus::prelude::rtu::attach_slave(transport, Slave(1));
//     read_voltage(ctx).await?;
//     Ok(())
// }
//
// fn main() {
//     let rt = Runtime::new().unwrap();
//     let addr = "192.168.68.53:502".parse().unwrap();
//
//     // Spawning a new thread
//     rt.spawn(async move {
//         continuous_voltage_read(addr)
//             .await
//             .expect("Failed to read voltage continuously");
//     });
//
//     // Keep the main thread alive indefinitely
//     loop {
//         std::thread::sleep(Duration::from_secs(10));
//     }
// }
// // use tokio::runtime::Runtime;
// // use tokio_modbus::prelude::*;
// //
// // async fn read_gude_controller() -> Result<(), Box<dyn std::error::Error>> {
// //     // Assuming the Gude Expert controller has the IP 192.168.0.100 and port 502
// //     let addr = "192.168.68.53:502".parse()?;
// //
// //     // Connect to the controller
// //     let mut client = tcp::connect(addr).await?;
// //
// //     // Address 0x404 for voltage, converting to Modbus index 0x404 (1028 decimal)
// //     let registers = client.read_input_registers(1028, 2).await?;
// //
// //     // Assuming the voltage values are in two registers as 32-bit data
// //     if registers.len() == 2 {
// //         let voltage = (i32::from(registers[0]) << 16) | i32::from(registers[1]); // Combine two registers into one 32-bit integer
// //         println!("Voltage: {} V", voltage as f32 / 100.0); // Assuming voltage scaling factor of 0.01 V as per documentation
// //     } else {
// //         println!("Error: Not enough registers read");
// //     }
// //
// //     Ok(())
// // }
// //
// // fn main() {
// //     let rt = Runtime::new().unwrap();
// //     rt.block_on(read_gude_controller())
// //         .expect("Failed to read from Gude Expert controller");
// // }
