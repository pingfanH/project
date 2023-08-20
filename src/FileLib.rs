use std::fs::File;
use std::io::Write;
use std::io::prelude::*;
use std::fs;
pub async fn writefile(path:&str,data:&[u8]) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}
pub async fn readfile(path: &str) -> Result<String, String> {
    match File::open(path) {
        Ok(mut file) => {
            // 读取文件内容
            let mut content = String::new();
            match file.read_to_string(&mut content) {
                Ok(_) => {
                    // 打印文件内容
                    println!("{}", content);
                    Ok(content)
                }
                Err(_) => {
                    println!("无法读取文件内容");
                    Err(String::from("Error"))
                }
            }
        }
        Err(e) => {
            println!("无法打开文件: {:?}", e);
            Err(String::from("Error"))
        }
    }
}