use serde_json::{json, Value};
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct CardData {
    pub result: ResultData,
    pub error: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct ResultData {
    pub cardId: u64,
    pub fields: FieldsData,
    pub fieldOrder: u64,
    pub question: String,
    pub answer: String,
    pub buttons: Vec<u64>,
    pub nextReviews: Vec<String>,
    pub modelName: String,
    pub deckName: String,
    pub css: String,
    pub template: String,
}

#[derive(Debug, Deserialize)]
pub struct FieldsData {
    pub Front: FieldValue,
    pub Back: FieldValue,
}

#[derive(Debug, Deserialize)]
pub struct FieldValue {
    pub value: String,
    pub order: u64,
}

pub fn request(action: &str, params: Option<serde_json::Map<String, Value>>) -> Value {
    let mut request_json = json!({
        "action": action,
        "version": 6
    });

    if let Some(p) = params {
        request_json["params"] = json!(p);
    }

    request_json
}

pub async fn invoke(action: &str, params: Option<serde_json::Map<String, Value>>) -> Result<String, Box<dyn Error>> {    
    let request_json = serde_json::to_vec(&request(action, params))?;

    let response = Client::new()
        .post("http://127.0.0.1:8765")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(request_json)
        .send()
        .await?;

    if response.status().is_success() {
        let body = response.text().await.unwrap();
        // let parsed_data: CardData = serde_json::from_str(&body)?;
        // Ok(parsed_data)
        Ok(body)
    } else {
        println!("Error");
        Err("Could not get data".into())
    }
}

pub fn parse_card(string_json: String) -> Result<CardData, Box<dyn Error>> {
    let parsed_data: CardData = serde_json::from_str(&string_json)?;
    Ok(parsed_data)
}

