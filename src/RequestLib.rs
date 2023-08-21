// use std::fs::File;
// use std::fs;
// use std::collections::HashMap;
use reqwest;
use reqwest::Error;
//use serde_json::Value;

pub async fn post(url:&str,body:&str){
    println!("{:#?}",body);
    let client = reqwest::Client::new();
    let _result=client.post(url)
        .body(reqwest::Body::from(body.to_string()))
        .send()
        .await;

}
pub async fn get(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}
