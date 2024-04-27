use std::{sync::Arc, time::Duration};

use rumqttc::{AsyncClient, MqttOptions, QoS};
use serde_derive::{Deserialize, Serialize};
use tokio::{sync::Mutex, time};

const BROKER_ADDRESS: &str = "localhost";
const BROKER_PORT: u16 = 1883;
const MQTT_ID: &str = "nebula_benchmark_client";
// Power reading mqtt topic
// const TOPIC: &str = "zwave/nodeID_23/meter/endpoint_0/value/66049";
// const TOPIC: &str = "de/gudesystems/epc/oh-my-gude/device/telemetry";
const TOPIC: &str = "zwave/Power/Smart_switch6/meter/endpoint_0/value/66049";

// Smart switch 6
#[derive(Serialize, Deserialize, Debug)]
pub struct TelemetryReading {
    pub time: u64,
    pub value: f64,
}

// Gude
#[derive(Debug, Deserialize, Serialize)]
struct LineIn {
    voltage: f64,
    current: f64,
    freq: f64,
    phase: f64,
    act_pow: f64,
    react_pow: f64,
    app_pow: f64,
    pow_fact: f64,
    tot_energy: f64,
    res_energy: f64,
    res_time: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct TelemetryReadingGude {
    line_in: Vec<LineIn>,
    ts: i64,
}

pub async fn read_from_mqtt(power_readings_for_mqtt: Arc<Mutex<Vec<TelemetryReading>>>) {
    println!("Didnt get here");
    let mut mqttoptions = MqttOptions::new(MQTT_ID, BROKER_ADDRESS, BROKER_PORT);
    mqttoptions.set_credentials("username", "pass");
    mqttoptions.set_keep_alive(Duration::from_secs(60));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    client
        .subscribe(TOPIC, QoS::AtLeastOnce)
        .await
        .expect("Failed to subscribe");

    println!("Subscribed to the topic: {}", TOPIC);

    loop {
        match eventloop.poll().await {
            Ok(notification) => {
                if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(publish)) = notification {
                    let payload_bytes = &publish.payload[..];
                    match serde_json::from_slice::<TelemetryReading>(payload_bytes) {
                        Ok(power_reading) => {
                            println!(
                                "Received power reading: {} at time {}",
                                power_reading.value,
                                power_reading.time,
                                // power_reading.line_in.first().unwrap().tot_energy,
                                // power_reading.ts
                            );
                            let mut readings = power_readings_for_mqtt.lock().await;
                            readings.push(power_reading);
                        }
                        Err(e) => {
                            eprintln!("Failed to parse JSON payload: {:?}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
        time::sleep(Duration::from_millis(100)).await;
    }
}

pub async fn wait_for_next_reading(power_readings: &Arc<Mutex<Vec<TelemetryReading>>>) {
    loop {
        let readings = power_readings.lock().await;
        if !readings.is_empty() {
            break;
        }
        drop(readings);
        time::sleep(Duration::from_secs(1)).await;
    }
}

pub async fn process_power_readings_for_module(
    power_readings: &Arc<Mutex<Vec<TelemetryReading>>>,
    module: &str,
) {
    let readings = power_readings.lock().await;

    println!("Processing readings for module: {}", module);

    for reading in readings.iter() {
        println!("{:?}", reading);
    }
}
