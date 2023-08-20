#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate serde_json;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use std::thread;
use slint::LogicalPosition;
use slint::SharedString;

//use tray_item::{IconSource, TrayItem};
//use crate::slint::SideBarItem::Component;
mod RequestLib;
mod json;
mod FileLib;
use serde_json::json;
slint::include_modules!();
#[derive(Debug)]
struct User {
    user_id: String,
    user_account: String,
    user_password: String,
    user_name: String,
    user_age: String,
    user_info: String,
    user_sign_date: String,
    user_gender: String,
    user_music_number: i32,
}
#[tokio::main]

async fn main() {
        let mut cookie="".to_string();
        match FileLib::readfile("C:/Program Files/P-layer/cookie/cookie.txt").await {
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
        user_id: user_data[0].clone().as_str().unwrap().to_string(),
        user_account: user_data[1].clone().as_str().unwrap().to_string(),
        user_password: user_data[2].clone().as_str().unwrap().to_string(),
        user_name: user_data[3].clone().as_str().unwrap().to_string(),
        user_age: user_data[4].clone().as_str().unwrap().to_string(),
        user_info: user_data[5].clone().as_str().unwrap().to_string(),
        user_sign_date: user_data[6].clone().as_str().unwrap().to_string(),
        user_gender: user_data[7].clone().as_str().unwrap().to_string(),
        user_music_number: user_data[8].clone().as_i64().unwrap() as i32,
    })
    .collect();

    let ui= AppWindow::new().unwrap();
    for user in user_array {
        ui.set_user_name(SharedString::from(user.user_name));
        ui.set_user_age(SharedString::from(user.user_age));
        ui.set_user_intro(SharedString::from(user.user_info));
        ui.set_user_gender(SharedString::from(user.user_gender));
        ui.set_user_music_number(user.user_music_number);
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
