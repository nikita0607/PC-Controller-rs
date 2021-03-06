extern crate reqwest;

use std::io::stdin;
use std::collections::HashMap;

use serde_json::{Value as JsonValue};
use serde_json::Error as JsonError;

use tokio::time::{Duration, sleep};


fn from_json(data: &str) -> Result<JsonValue, JsonError>{

    let json = serde_json::from_str(data);

    json

}

struct Computer {
    user_name: String,
    name: String,
    addr: String,
    client: reqwest::Client
}

impl Computer {

    fn new(client: reqwest::Client, user_name: String, name: String, addr: String) -> Computer {
        let addr = addr.replace("\n","");
        Computer {user_name, name, addr, client}
    }

    fn get_empty_json(&self) -> String {
        std::format!("\"{}\": \"{}\", \"{}\": \"{}\", \"{}\": \"{}\"",
            "computer_name", &self.name.as_str(),
            "user_name", &self.user_name.as_str(),
            "adr", "Hello"
        )
    }

    fn json_constructor(&self, data: HashMap<String, String>) -> Result<JsonValue, JsonError> {
        let mut json = self.get_empty_json();

        for (i, v) in data.iter() {
            json += std::format!(", \"{}\": \"{}\"", i, v).as_str();
        };

        let json= std::format!("{}{}{}", "{", json, "}");

        from_json(&json)
    }

    async fn make_response(&self, data: JsonValue) -> Result<Result<JsonValue, JsonError>, reqwest::Error> {
        let response_text = self.client.post(&self.addr)
            .json(&data)
            .send()
            .await?
            .text()
            .await?;

        println!("{}", response_text.as_str());

        Ok(from_json(response_text.as_str()))
    }

    async fn empty_response(&self) -> Result<Result<JsonValue, JsonError>, reqwest::Error>{

        let mut _data: HashMap<String, String> = HashMap::new();
        _data.insert("method".to_string(), "".to_string());
        _data.insert("get_next".to_string(), "true".to_string());

        let res = self.json_constructor(_data);

        match res {
            Ok(data) => {
                Ok(self.make_response(data).await?)
            },

            JsonError=> Ok(JsonError)
        }


    }

    async fn add_button(&self, _name: &str, _text: &str) -> Result<Result<JsonValue, JsonError>, reqwest::Error> {

        let mut _data: HashMap<String, String> = HashMap::new();
        _data.insert("method".to_string(), "button.add".to_string());
        _data.insert("name".to_string(), _name.to_string());
        _data.insert("text".to_string(), _text.to_string());

        let res = self.json_constructor(_data);

        match res {
            Ok(data) => {
                Ok(self.make_response(data).await?)
            },

            JsonError=> Ok(JsonError)
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    println!("?????????????? ?????????? ????????????:");

    let mut _addr = String::new();
    stdin().read_line(&mut _addr)?;

    let addr = std::format!("{}{}{}", "http://192.168.", _addr.as_str(), ":5000/a");

    let comp = Computer::new(
        client,
        "nikita0607".to_string(),
        "test".to_string(),
        addr.to_string()
    );

    comp.add_button("Test", "test").await?;

    loop {
        let json = comp.empty_response().await??;

        println!("{:?}", json);

        sleep(Duration::from_millis(2000)).await;
    }

    Ok(())
}
