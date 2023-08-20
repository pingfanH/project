// use std::fs::File;
// use std::fs;
// use std::collections::HashMap;
use reqwest;
use reqwest::{Client, Error};
use serde_json::Value;

pub async fn post(url:&str,body:&str){
    let client = reqwest::Client::new();
    client.post(url)
        .body(reqwest::Body::from(body.to_string()))
        .send()
        .await;
}
pub async fn get(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}
pub async fn get_json(url: &str) -> Result<Value, Error>  {
    let client = Client::new();
    let response = client.get(url).send().await?;
    println!("{:#?}",response);
    let json: Value = response.json().await?;
    
    Ok(json)
}