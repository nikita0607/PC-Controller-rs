extern crate reqwest;

use std::io::{stdin, Read};

use serde_json::Value as JsonValue;
use serde_json::Error as JsonError;


fn get_json(data: &str) -> Result<JsonValue, JsonError>{

    let v: JsonValue = serde_json::from_str(data)?;

    Ok(v)
}

async fn make_response(client: reqwest::Client, addr: &str) -> Result<String, reqwest::Error> {

    println!("{}", addr);


    let response_text = client.post(addr)
        .json(&serde_json::json!({
            "computer_name": "Test",
            "user_name": "nikita0607",
			"adr": addr,
			"method": ""
        }))
        .send()
        .await?
        .text()
        .await?;

    Ok(response_text)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    println!("Введите часть адреса:");

    let mut _addr = String::new();
    stdin().read_line(&mut _addr)?;

    let addr = std::format!("{}{}{}", "http://192.168.", _addr.as_str(), ":5000/a");

    let response_text = make_response(client, addr.as_str()).await?;

    let json = get_json(response_text.replace("\n", "").as_str())?;

    println!("{:?}", json);

    Ok(())
}
