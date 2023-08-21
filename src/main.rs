#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate serde_json;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use std::thread;
use slint::LogicalPosition;
use slint::SharedString;
use std::collections::HashMap;
//use tray_item::{IconSource, TrayItem};
//use crate::slint::SideBarItem::Component;
mod RequestLib;
mod json;
mod FileLib;
use serde_json::json;
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
#[tokio::main]

async fn main() {
        let mut cookie="".to_string();
        match FileLib::readfile("C:/Program Files/P-layer/cookies/.token").await {
            Ok(content) => cookie=content,
            Err(e) => println!("{}", e),
        };
        println!("{}",cookie);
        //let mut user_data=json!({"name": "pingfanh"});
        let mut user_data: String = String::new();
        let mut url=format!("http://127.0.0.1:8000/api/query_user/{}",cookie);
        match RequestLib::get(url.as_str()).await {
            Ok(body) => {
                println!("{body}");
                user_data = body;
            },
            Err(err) => eprintln!("Error: {}", err),
        }
        
        let json_array: Vec<Vec<serde_json::Value>> = serde_json::from_str(user_data.as_str()).unwrap();

let user_array: Vec<User> = json_array
    .into_iter()
    .map(|user_data| User {
        user_token: user_data[0].clone().as_str().unwrap().to_string(),
        user_id: user_data[1].clone().as_str().unwrap().to_string(),
        user_account: user_data[2].clone().as_str().unwrap().to_string(),
        user_password: user_data[3].clone().as_str().unwrap().to_string(),
        user_name: user_data[4].clone().as_str().unwrap().to_string(),
        user_gender: user_data[5].clone().as_str().unwrap().to_string(),
        user_age: user_data[6].clone().as_str().unwrap().to_string(),
        user_intro: user_data[7].clone().as_str().unwrap().to_string(),
        user_sign_date: user_data[8].clone().as_str().unwrap().to_string(),
        user_music_number: user_data[9].clone().as_i64().unwrap() as i32,
    })
    .collect();



    let ui= AppWindow::new().unwrap();
    for user in user_array {

        FileLib::creat_cookie("id", user.user_id.as_bytes()).await;
        FileLib::creat_cookie("name", user.user_name.as_bytes()).await;
        FileLib::creat_cookie("age", user.user_age.as_bytes()).await;
        FileLib::creat_cookie("intro", user.user_intro.as_bytes()).await;
        FileLib::creat_cookie("gender", user.user_gender.as_bytes()).await;
        FileLib::creat_cookie("sign_date", user.user_sign_date.to_string().as_bytes()).await;

        ui.set_user_id(SharedString::from(user.user_id));
        ui.set_user_name(SharedString::from(user.user_name));
        ui.set_user_age(SharedString::from(user.user_age));
        ui.set_user_intro(SharedString::from(user.user_intro));
        ui.set_user_gender(SharedString::from(user.user_gender));
        ui.set_user_music_number(user.user_music_number);
        ui.set_user_sign_date(SharedString::from(user.user_sign_date));

    }
    

    let handle = ui.as_weak();
    let hanel_close = ui.as_weak();
    
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
    ui.on_apply( move |updated_user_data|{

        let mut user_data = HashMap::new();
        user_data.insert("user_account", cookie.as_str());
        user_data.insert("user_name", updated_user_data.name.as_str());
        user_data.insert("user_age", updated_user_data.age.as_str());
        user_data.insert("user_intro", &updated_user_data.intro.as_str());
        user_data.insert("user_gender", &updated_user_data.gender.as_str());

        let user_data = vec![json!(user_data)];
        let body = serde_json::to_string(&user_data).unwrap();
        println!("{:#?}", body);
        tokio::spawn(async move {
            RequestLib::post("http://127.0.0.1:8000/api/user/update", body.as_str()).await;
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


async fn CheckUser(){

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
