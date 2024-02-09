use reqwest::Client;
// use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let url = "http://nebula.no/api/wasm";

    let client = Client::new();

    let _ = make_request(&client, url, "10").await;
}

async fn make_request(client: &Client, url: &str, input_value: &str) -> reqwest::Result<()> {
    let payload = [
        ("function_name", "fibonacci"),
        ("module_type", "Wasm"),
        ("input", input_value),
    ];

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
