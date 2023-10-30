use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::io::prelude::*;
use std::fs;
use std::path::Path;

use serde_json::Value;
use crate::structs::MusicList;

pub async fn writefile(path: &str, data: &[u8]) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    match readfile(path).await{
        Ok(_) => {},
        Err(_) => {
            {let _ = fs::remove_file(path);}}
    }

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
pub async fn creat_cookie(name:&str,data:&[u8]){
    let path=format!("C:/Program Files/P-layer/cookies/.{}",name);
    let _ = writefile(path.as_str(), data.as_ref()).await;
}
pub async fn read_cookie(name:&str)-> Result<String, String>{
    let path=format!("C:/Program Files/P-layer/cookies/.{}",name);
    match File::open(path) {
        Ok(mut file) => {
            // 读取文件内容
            let mut content = String::new();
            match file.read_to_string(&mut content) {
                Ok(_) => {
                    // 打印文件内容
                    let content=content.trim_matches('"');
                    println!("{}", content);
                    Ok(content.to_string())
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

pub async fn readfilenameloop(path:&str)->Vec<String>{
    let mut jsonlist:Vec<MusicList>=vec![];
    let folder_path = Path::new(path);
    let mut lists:Vec<String>=vec![];
    // 检查文件夹是否存在
    if folder_path.is_dir() {
        // 获取文件夹内所有项的迭代器
        if let Ok(entries) = fs::read_dir(folder_path) {
            // 遍历每个项
            for entry in entries {
                if let Ok(entry) = entry {
                    // 获取项的名称
                    let entry_name = entry.file_name();

                    // 将名称转换为字符串
                    if let Some(name) = entry_name.to_str() {
                        //println!("{}", name);
                        let name=name.to_string();
                        lists.push(name);
                    }
                }
            }
        }
    } else {
        println!("文件夹不存在");
    }

    // for list in lists {
    //     println!("list:{}",list);
    //     let list:Vec<&str>=list.split(",").collect();
    //     let musicdata:MusicList=MusicList { user: list[0].to_owned(), name: list[1].to_owned(), date: list[2].to_owned(), public: str2bool(list[3]).unwrap() };
    //     jsonlist.push(musicdata);

    // };
    lists
}

//循环一个vec<Value>,当select和value的值在json中能对照时返回这些json:vec<Value>
async fn selectfromjson(jsonlist:Vec<serde_json::Value>,select:&str,value:&str)->Vec<serde_json::Value>{

    let mut selectedjson:Vec<Value>= vec![];
    for i in jsonlist{
            if i[select]==value{
                selectedjson.push(i);
            }
    };
    selectedjson
}
pub fn str2bool(s: &str) -> Option<bool> {
    match s.to_lowercase().as_str() {
        "true" | "t" | "yes" | "y" | "1" => Some(true),
        "false" | "f" | "no" | "n" | "0" => Some(false),
        _ => None,
    }
}
pub async fn copyfile(src_path: &str, dest_path: &str) -> io::Result<()> {
    let src_file = File::open(src_path)?;
    let dest_file = File::create(dest_path)?;

    let mut reader = BufReader::new(src_file);
    let mut writer = BufWriter::new(dest_file);

    io::copy(&mut reader, &mut writer)?;

    Ok(())
}