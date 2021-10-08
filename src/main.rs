extern crate reqwest;

use std::io::{stdin, stdout};
use serde_json::Value as JsonValue;

fn make_respone(addr: &str) -> Result<JsonValue, Box<dyn std::error::Error>> {

    let response_text: std::string::String = client.post(addr)
        .json(&serde_json::json!({
            "computer_name": "Test",
            "user_name": "nikita0607",
			"adr": "192.168.18.219",
			"method": ""
        }))
        .send()
        .await?
        .text()
        .await?;

    serde_json::from_str(&response_text.as_str())?
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut _addr = String::new();
    stdin().read_line(&mut _addr);

    let addr = std::format!("{}{}{}", "http://192.168.", _addr.as_str(), ":5000/a").as_str();

    let v: JsonValue = make_respone(&addr).unwrap();

    let count = v.get("count").unwrap().as_u64().unwrap();

    println!("{:?}", count);

    Ok(())
}
