extern crate reqwest

use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let answer = client.post("http://192.168.20.178:5000/a")
        .json(&serde_json::json!({
            "computer_name": "Test",
            "user_name": "nikita0607",
			"adr": "192.168.18.219",
			"method": ""
        }))
        .send()
        .await?
		.json()
		.await?;

    println!("{:#?}", answer);

    Ok(())
}
