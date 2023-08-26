#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate serde_json;
use std::fs::File;
use std::io::BufReader;
use std::path;
use std::path::Path;
use std::process;
use rodio::{Decoder, OutputStream, Sink};
use std::thread;
use slint::LogicalPosition;
use slint::SharedString;
use std::collections::HashMap;
use std::fs;

use rfd::FileDialog;

use std::sync::mpsc;
use tray_item::{IconSource, TrayItem};

mod RequestLib;
mod json;
mod FileLib;
mod slinttrait;
mod musicplayer;

use serde_json::json;
use std::sync::{Arc, RwLock};
use std::cell::Cell;
use std::sync::mpsc::{channel, Receiver, Sender};
use tokio::task;

use crate::slinttrait::CargoMessage;
slint::include_modules!();
#[derive(Debug)]
struct User {
    user_token:String,
    user_id: String,
    user_account: String,
    user_password: String,
    user_name: String,
    user_gender: String,
    user_age: String,
    user_intro: String,
    user_sign_date: String,
    user_music_number: i32,
}
enum Message {
    Quit,
    LogOff,
    Red,
    Run,
    Test
}
#[tokio::main]

async fn main() {
    let ui= AppWindow::new().unwrap();
    let hanel_close = ui.as_weak();
        let mut tray = TrayItem::new(
            "P-layer",
            IconSource::Resource("name-of-icon-in-rc-file"),
        )
        .unwrap();
    
        tray.add_label("P-layer").unwrap();
        
    
        tray.inner_mut().add_separator().unwrap();
    
        let (tx, rx) = mpsc::sync_channel(1);
        let run_tx=tx.clone();
        tray.add_menu_item("启动", move|| {
            run_tx.send(Message::Run).unwrap();
           
        })
        .unwrap();

        let red_tx = tx.clone();
        tray.add_menu_item("退出登录", move || {
            red_tx.send(Message::LogOff).unwrap();
        })
        .unwrap();

        let red_tx = tx.clone();
        tray.add_menu_item("test", move || {
            red_tx.send(Message::Test).unwrap();
        })
        .unwrap();
    
        tray.inner_mut().add_separator().unwrap();
    
        let quit_tx = tx.clone();
        tray.add_menu_item("退出", move || {
            
            quit_tx.send(Message::Quit).unwrap();
        })
        .unwrap();
        
            loop {
                match rx.recv() {
                    Ok(Message::Quit) => {
                        println!("Quit");
                        hanel_close.upgrade().unwrap().hide().unwrap();
                        process::exit(0);
                    }
                    Ok(Message::Red) => {
                        println!("Red");
                        tray.set_icon(IconSource::Resource("another-name-from-rc-file"))
                            .unwrap();
                        
                    }
                    Ok(Message::LogOff) => {
                        println!("Logoff");
                        match delete_folder_contents("C:/Program Files/P-layer/cookies") {
                            Ok(()) => println!("del"),
                            Err(e) => println!("del err:{}", e),
                        }
                    }
                    Ok(Message::Test) => {
                        println!("Test");
                        ui.set_user_name(SharedString::from("1234"));
                    }
                    Ok(Message::Run) => {
                        Apprun(&ui).await;
                    }
                    _ => {}
                }
            }



}

async fn Apprun(ui:&AppWindow){
   // let ui= AppWindow::new().unwrap();
    //let cargo_worker = slint::CargoWorker::new(&cargo_ui);
    let handle = ui.as_weak();
    let hanel_close = ui.as_weak();
    
    let set_values=ui.as_weak();
    let cargo_worker = slinttrait::CargoWorker::new(&ui);
        let mut istoken:bool=false;

        let mut cookie="".to_string();
        match FileLib::readfile("C:/Program Files/P-layer/cookies/.token").await {
            Ok(content) => {{
                cookie=content;
                istoken=true;
            }},
            Err(e) => println!("{}", e),
        };
        println!("{}",cookie);
        // let mut user_data: String = String::new();
        // let mut url=format!("http://127.0.0.1:8000/api/query_user/{}",cookie);
       
        // match RequestLib::get(url.as_str()).await {
        //     Ok(body) => {
        //         println!("{body}");
        //         user_data = body;
        //     },
        //     Err(err) => eprintln!("Error: {}", err),
        // }

        if(istoken){

                    ui.set_is_login(true);
                    match FileLib::read_cookie("name").await{
                        Ok(ok)=>{ui.set_user_name(SharedString::from(ok))},
                        Err(err)=>eprintln!("{}",err)
                    }
                    match FileLib::read_cookie("id").await{
                        Ok(ok)=>ui.set_user_id(SharedString::from(ok)),
                        Err(err)=>eprintln!("{}",err)
                    }
                    match FileLib::read_cookie("age").await{
                        Ok(ok)=>ui.set_user_age(SharedString::from(ok)),
                        Err(err)=>eprintln!("{}",err)
                    }
                    match FileLib::read_cookie("intro").await{
                        Ok(ok)=>ui.set_user_intro(SharedString::from(ok)),
                        Err(err)=>eprintln!("{}",err)
                    }
                    match FileLib::read_cookie("gender").await{
                        Ok(ok)=>ui.set_user_gender(SharedString::from(ok)),
                        Err(err)=>eprintln!("{}",err)
                    }
                    match FileLib::read_cookie("sign_date").await{
                        Ok(ok)=>ui.set_user_sign_date(SharedString::from(ok)),
                        Err(err)=>eprintln!("{}",err)
                    }
                    match FileLib::read_cookie("music_number").await{
                        Ok(ok)=>ui.set_user_music_number(ok.parse().unwrap()),
                        Err(err)=>eprintln!("{}",err)
                    }
        };



    

   
    
    //监听按键
    // thread::spawn(|| {
    //     let device_state = DeviceState::new();
    //     let keys: Vec<Keycode> = device_state.get_keys();
    //     loop{
    //         //if keys.contains(&Keycode::A)
    //         //{

    //             println!("{}",keys.contains(&Keycode::A));
    //         //}
    //     }
    // });
    
    //关闭窗口

    ui.on_close_window(move ||{
        musicplay("C:/Program Files/P-layer/data/audios/notes.mp3");
        hanel_close.upgrade().unwrap().hide().unwrap();
    });
    //跳转到url
    ui.on_open_the_url(|url|{
        println!("{}",url); 
        open::that(url.as_str()).ok();
    });
    //播放点击音效1,2
    ui.on_click_audio(move ||{
        thread::spawn(|| {
            musicplay("C:/Program Files/P-layer/data/audios/keypress.mp3");
        });
    });

    ui.on_click_audio1(move ||{
        thread::spawn(|| {
            musicplay("C:/Program Files/P-layer/data/audios/notes.mp3");
        });
    });
    ui.on_test(move|value|{
        let cargo_channel = cargo_worker.channel.clone();
        cargo_channel.send(CargoMessage::SetValue("set_user_name".to_string(),value.to_string())).unwrap()
    });
    ui.on_musicupload(move|name|{
        let files = FileDialog::new()
        .add_filter("music", &["mp3","mp4","flac","wav","ogg","avi","mov","mkv"])
        //.add_filter("rust", &["rs", "toml"])
        .set_directory("/")
        .pick_file();
    print!("name{}",name.as_str());
    if let Some(files ) = files{
            let path=files.display().to_string();
            // let path=path.as_str().replace("\\", "/");
            println!("{}",path);
           
            tokio::spawn(async move{
                match  FileLib::read_cookie("account").await{
                    Ok(ok)=>{
                        let name=name.as_str();
                        let filename=path.split(r"\").last().unwrap();
                        println!("{}",name);
                        RequestLib::upload(path.as_str(),"http://127.0.0.1:8000/api/upload", ok.as_str(),filename).await;
                    },
                    Err(err)=>eprintln!("{}",err)
                }
            });
    }
    });
    let cargo_worker = Arc::new(slinttrait::CargoWorker::new(&ui));
    let cargo_channel = cargo_worker.channel.clone();
    ui.on_choose_avatar(move||{
        let cargo_channel = cargo_channel.clone();
        let files = FileDialog::new()
        .add_filter("music", &["png"])
        .set_directory("/")
        .pick_file();
    if let Some(files ) = files{
            let path=files.display().to_string();
            // let path=path.as_str().replace("\\", "/");
            println!("{}",path);
            let cargo_channel = cargo_worker.channel.clone();
            cargo_channel.send(CargoMessage::SetValue("set_avatar_path".to_string(),path)).unwrap();

        };
       


    });
    ui.on_apply( move |updated_user_data|{
        
        let mut user_data = HashMap::new();
        user_data.insert("user_token", cookie.as_str());
        user_data.insert("user_name", updated_user_data.name.as_str());
        user_data.insert("user_age", updated_user_data.age.as_str());
        user_data.insert("user_intro", &updated_user_data.intro.as_str());
        user_data.insert("user_gender", &updated_user_data.gender.as_str());

        let user_data = vec![json!(user_data)];
        let body = serde_json::to_string(&user_data).unwrap();
        println!("apply {:#?}", body);
            let name = updated_user_data.name;
            let age = updated_user_data.age.clone();
            let intro = updated_user_data.intro.clone();
            let gender = updated_user_data.gender.clone();
        tokio::spawn(async move {
           
            FileLib::creat_cookie("name", name.as_bytes()).await;
            FileLib::creat_cookie("age", age.as_bytes()).await;
            FileLib::creat_cookie("intro", intro.as_bytes()).await;
            FileLib::creat_cookie("gender", gender.as_bytes()).await;
            RequestLib::post("http://127.0.0.1:8000/api/user/update", body.as_str()).await;
        });
    });
    let cargo_worker = Arc::new(slinttrait::CargoWorker::new(&ui));
    let cargo_channel = cargo_worker.channel.clone();
    ui.on_login( move|account,password|{

       
        let cargo_channel = cargo_channel.clone();
        tokio::spawn(async move {
            //ui.set_user_name(SharedString::from("123"));
            let account:&str=account.as_str();
            let password:&str=password.as_str();
            //println!("{},{}",account,password);
            let mut user_data=format!("http://127.0.0.1:8000/api/get_token/{}/{}",account,password);
            match  RequestLib::get(user_data.as_str()).await{
                Ok(ok)=>{
                    user_data=ok;
                },
                Err(err)=>eprint!("{}",err)
            } 
            println!("{}",user_data);

            let mut user_data=json::str2json(user_data.as_str());
            
            cargo_channel.send(CargoMessage::SetValue("set_user_name".to_string(),user_data["user_name"].to_string().replace('"',""))).unwrap();
            cargo_channel.send(CargoMessage::SetValue("set_user_age".to_string(),user_data["user_age"].to_string().replace('"',""))).unwrap();
            cargo_channel.send(CargoMessage::SetValue("set_user_id".to_string(),user_data["user_id"].to_string().replace('"',""))).unwrap();
            cargo_channel.send(CargoMessage::SetValue("set_user_gender".to_string(),user_data["user_gender"].to_string().replace('"',""))).unwrap();
            cargo_channel.send(CargoMessage::SetValue("set_user_intro".to_string(),user_data["user_intro"].to_string().replace('"',""))).unwrap();
            cargo_channel.send(CargoMessage::SetValue("set_user_music_number".to_string(),user_data["user_music_number"].to_string().replace('"',""))).unwrap();
            cargo_channel.send(CargoMessage::SetValue("set_user_sign_date".to_string(),user_data["user_sign_date"].to_string().replace('"',""))).unwrap();
            
            //let mut name = Arc::new(user_data["user_name"].to_string().clone());
            FileLib::creat_cookie("token", user_data["user_token"].to_string().replace('"',"").as_bytes()).await;
            FileLib::creat_cookie("account", user_data["user_account"].to_string().replace('"',"").as_bytes()).await;
            FileLib::creat_cookie("password", user_data["user_password"].to_string().replace('"',"").as_bytes()).await;
            FileLib::creat_cookie("id", user_data["user_id"].to_string().replace('"',"").as_bytes()).await;
            FileLib::creat_cookie("name", user_data["user_name"].to_string().replace('"',"").as_bytes()).await;
            FileLib::creat_cookie("age", user_data["user_age"].to_string().replace('"',"").as_bytes()).await;
            FileLib::creat_cookie("intro", user_data["user_intro"].to_string().replace('"',"").as_bytes()).await;
            FileLib::creat_cookie("gender", user_data["user_gender"].to_string().replace('"',"").as_bytes()).await;
            FileLib::creat_cookie("sign_date", user_data["user_sign_date"].to_string().replace('"',"").as_bytes()).await;
            FileLib::creat_cookie("music_number", user_data["user_music_number"].to_string().replace('"',"").as_bytes()).await;
    


            match FileLib::read_cookie("account").await{
                Ok(ok)=> RequestLib::download("http://127.0.0.1:8000/api/download",&ok.as_str(),"avatar.png").await,
                Err(err)=>eprintln!("{}",err)
            }

           
        });
    
    });

    ui.on_sign(move|SignData|{
        

        let body=format!("{},{},{},{},{},{}", SignData.account.as_str(),SignData.password.as_str(),SignData.name.as_str(),SignData.gender.as_str(),SignData.age.as_str(),SignData.intro.as_str());
        //let user_name=SignData.name;
        tokio::spawn(async move {
            RequestLib::post("http://127.0.0.1:8000/api/user/create",body.as_str()).await;
            FileLib::creat_cookie("account", SignData.account.as_bytes()).await;
            FileLib::creat_cookie("password", SignData.password.as_bytes()).await;
            FileLib::creat_cookie("name", SignData.name.as_bytes()).await;
            FileLib::creat_cookie("gender", SignData.gender.as_bytes()).await;
            FileLib::creat_cookie("age", SignData.age.as_bytes()).await;
            FileLib::creat_cookie("intro", SignData.intro.as_bytes()).await;
            let mut user_data=format!("http://127.0.0.1:8000/api/get_token/{}/{}",SignData.account,SignData.password);
            match  RequestLib::get(user_data.as_str()).await{
                Ok(ok)=>{
                    user_data=ok;
                },
                Err(err)=>eprint!("{}",err)
            
            } 
        });
       
    });
    //播放音乐
    ui.on_music_play(move ||{
        thread::spawn(|| {
            musicplay("C:/Program Files/P-layer/data/music/Guitar, Loneliness and Blue Planet.mp3");
        });
    });
    
    // ui.on_music_pause(move ||{
    //     thread::spawn(|| {
    //         musicpause(sink)
    //     });
    // });

    //移动窗口
    ui.on_move_window(move |offset_x, offset_y|{
        let main = handle.upgrade().unwrap();
        //获取窗口的物理坐标
        let logical_pos = main.window().position().to_logical(main.window().scale_factor());
        //窗口坐标添加上偏移量，设置为新的位置
        main.window().set_position(LogicalPosition::new(logical_pos.x + offset_x, logical_pos.y + offset_y));
       
    });
    
    let _ = ui.run();
}

fn delete_folder_contents(folder_path: &str) -> std::io::Result<()> {
    let path = Path::new(folder_path);

    // 遍历文件夹内的内容
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        // 如果是文件，则直接删除
        if entry_path.is_file() {
            fs::remove_file(entry_path)?;
        }
        // 如果是文件夹，则递归调用自身进行删除
        else if entry_path.is_dir() {
            delete_folder_contents(entry_path.to_str().unwrap())?;
            fs::remove_dir(entry_path)?;
        }
    }

    Ok(())
}
fn musicplay(path: &str){
    println!("{}",path);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
         let file = BufReader::new(File::open(path).unwrap());
    // Decode that sound file into a source
         let source = Decoder::new(file).unwrap();

    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);
    // Play the sound directly on the device
         //let _player = stream_handle.play_raw(source.convert_samples());
         sink.sleep_until_end();
         
    // The sound plays in a separate audio thread,
         // so we need to keep the main thread alive while it's playing.
         //std::thread::sleep(std::time::Duration::from_secs(1));
}

// fn musicpause(sink: &Sink) {
//     sink.pause();
// }
