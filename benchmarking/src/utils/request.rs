use std::time::Duration;

use reqwest::Client;
use tokio::time::Instant;

const URL: &str = "http://192.168.68.69/api/wasm_headless";
// const URL: &str = "http://raspberrypi.local/api/wasm";
// const URL: &str = "http://nebula.no/api/wasm";

pub async fn bombard_nebula(client: Client) {
    let modules: Vec<&str> = vec![
        // "exponential",
        // "factorial",
        "fibonacci",
        // "fibonacci-recursive",
        //"prime-number",
    ];
    // let base_images = ["debian", "ubuntu", "archlinux"];
    let base_images = ["debian"];
    let duration_per_function = Duration::new(60, 0);

    for module in modules {
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
                        "Wasm",
                        &input_value.to_string(),
                        image,
                    )
                    .await;
                }

                // sleep(Duration::from_millis(100)).await;
                // }
            }
        }
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
