use reqwest::Client;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use serde_derive::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::process;
use std::{
    fs::{self, File},
    io::{self, Read},
    path::PathBuf,
    sync::Arc,
    time::Instant,
};
use tokio::{
    sync::Mutex,
    time::{self, sleep, Duration},
};

#[derive(Serialize, Deserialize, Debug)]
struct PowerReading {
    time: u64,
    value: f64,
}

async fn read_baseline() -> io::Result<Vec<f64>> {
    let baseline_path = get_data_path("baseline_readings");
    let mut file = File::open(baseline_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let results = serde_json::from_str(&contents)?;
    Ok(results)
}

async fn write_baseline(readings: &Vec<f64>) -> io::Result<()> {
    let baseline_path = get_data_path("baseline_readings");

    let serialized = serde_json::to_string(readings)?;
    fs::write(baseline_path, serialized)?;
    Ok(())
}

const VERSION: &str = "0.3.1";
fn get_data_path(file_name: &str) -> PathBuf {
    let home_dir = dirs::home_dir().expect("Home directory not found");
    let app_dir = home_dir.join(".nebula");
    let vers_dir = app_dir.join(VERSION);

    fs::create_dir_all(&vers_dir).expect("Failed to create version directory");

    vers_dir.join(format!("{}.json", file_name))
}

#[tokio::main]
async fn main() {
    let baseline = read_baseline().await.unwrap();
    let power_readings: Arc<Mutex<Vec<f64>>> = Arc::new(Mutex::new(baseline));

    let power_readings_for_mqtt = power_readings.clone();

    let mqtt_handle = tokio::spawn(async move {
        read_from_mqtt(power_readings_for_mqtt).await;
    });

    // let client = Client::new();

    // let bombard_handle = tokio::spawn(async move {
    //     bombard_nebula(client, power_readings).await;
    // });
    let measure_baseline_handle = tokio::spawn(async move {
        time::sleep(Duration::from_secs(20 * 60)).await;
        let readings = power_readings.lock().await;
        let _ = write_baseline(&readings).await;
    });
    println!("After async");

    let _ = tokio::join!(mqtt_handle, measure_baseline_handle);
}

const BROKER_ADDRESS: &str = "localhost";
const BROKER_PORT: u16 = 1883;
const MQTT_ID: &str = "nebula_benchmark_client";
// Power reading mqtt topic
const TOPIC: &str = "zwave/nodeID_23/meter/endpoint_0/value/66049";

async fn read_from_mqtt(power_readings_for_mqtt: Arc<Mutex<Vec<f64>>>) {
    println!("Didnt get here");
    let mut mqttoptions = MqttOptions::new(MQTT_ID, BROKER_ADDRESS, BROKER_PORT);
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
                    match serde_json::from_slice::<PowerReading>(payload_bytes) {
                        Ok(power_reading) => {
                            println!(
                                "Received power reading: {} at time {}",
                                power_reading.value, power_reading.time
                            );
                            let mut readings = power_readings_for_mqtt.lock().await;
                            readings.push(power_reading.value);
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

async fn wait_for_next_reading(power_readings: &Arc<Mutex<Vec<f64>>>) {
    loop {
        let readings = power_readings.lock().await;
        if !readings.is_empty() {
            break;
        }
        drop(readings);
        time::sleep(Duration::from_secs(1)).await;
    }
}

async fn process_power_readings_for_module(power_readings: &Arc<Mutex<Vec<f64>>>, module: &str) {
    let readings = power_readings.lock().await;

    println!("Processing readings for module: {}", module);

    for reading in readings.iter() {
        println!("{}", reading);
    }
}

const URL: &str = "http://raspberrypi.local/api/wasm";
// const URL: &str = "http://nebula.no/api/wasm";
//
async fn bombard_nebula(client: Client, power_readings: Arc<Mutex<Vec<f64>>>) {
    // let url = "http://raspberrypi.local/api/wasm";

    let modules: Vec<&str> = vec![
        // "exponential",
        // "factorial",
        "fibonacci",
        // "fibonacci-recursive",
        //"prime-number",
    ];
    // let base_images = ["debian", "ubuntu", "archlinux"];
    let base_images = ["debian"];
    let duration_per_function = Duration::new(5 * 60, 0);

    for module in modules {
        wait_for_next_reading(&power_readings).await;
        let start_time = Instant::now();
        while Instant::now().duration_since(start_time) < duration_per_function {
            for input_value in 0..=45 {
                // for _ in 0..9 {
                // let _ =
                //     make_request(&client, URL, module, "Wasm", &input_value.to_string(), "").await;
                for image in base_images {
                    let _ = make_request(
                        &client,
                        URL,
                        module,
                        "Docker",
                        &input_value.to_string(),
                        image,
                    )
                    .await;
                }

                // sleep(Duration::from_millis(100)).await;
                // }
            }
        }
        process_power_readings_for_module(&power_readings, module).await;
    }
}

async fn make_request(
    client: &Client,
    url: &str,
    module: &str,
    module_type: &str,
    input_value: &str,
    base_image: &str,
) -> reqwest::Result<()> {
    let payload = [
        ("function_name", module),
        ("module_type", module_type),
        ("input", input_value),
        ("base_image", base_image),
    ];

    println!("making req");

    let resp = client.post(url).form(&payload).send().await?;

    if resp.status().is_success() {
        println!("Request for input {} successful", input_value);
    } else {
        let error = resp.error_for_status();
        eprintln!(
            "Request for input {} failed, message: {:?}",
            input_value, error
        );
    }

    Ok(())
}
