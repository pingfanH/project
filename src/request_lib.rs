// use std::fs::File;
use std::fs;
use std::fs::File;
use std::io::Write;
use futures::FutureExt;
// use std::collections::HashMap;
use reqwest;
use reqwest::Error;
use reqwest::Client;
//use serde_json::Value;

pub async fn post(url:&str,body:&str){
    //println!("{:#?}",body);
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
pub async fn upload(path:&str,url:&str)->Result<String, Box<dyn std::error::Error>>{

   // println!("{}",url);
    // 创建 HTTP 客户端
    let client = Client::new();

    // 读取二进制文件
    let file_content = fs::read(path)?;
    // 发送 POST 请求
    let post = client.post(url)
    .body(file_content)
    .send()
    .await;
    // 发送 POST 请求
    let text=post.unwrap().text().await.unwrap();
    //println!("{:?}",response);
    Ok(text)
}
pub async fn download(url:&str,targe:&str)->String{
    // 创建目标文件并写入内容
   let response = reqwest::get(url).await.expect("Failed to send GET request");
   let content = response.bytes().await.expect("Failed to retrieve response content");
    println!("targe:{}",targe);
   let mut target_file = File::create(targe).expect("Failed to create target file");
   target_file.write_all(&content).expect("Failed to write content to target file");
   "success".to_owned()
}

pub async fn get_file(url: &str) -> Result<Vec<u8>, Error> {
    let response = reqwest::get(url).await?;
    let mut buf = Vec::new();
    response
        .bytes()
        .map(|chunk| {
            let chunk = chunk.unwrap();
            buf.extend_from_slice(&chunk);
        })
        .await;
    Ok(buf)
}

pub async fn downloadpubmusic(url:&str,filename:&str,targe:&str){
    // 创建目标文件并写入内容
   let url=format!("{}/{}",url,filename);
   let response = reqwest::get(&url).await.expect("Failed to send GET request");
   let content = response.bytes().await.expect("Failed to retrieve response content");

   let mut target_file = File::create(targe).expect("Failed to create target file");
   target_file.write_all(&content).expect("Failed to write content to target file");
}