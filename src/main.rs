use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use std::io::prelude::*;
use std::thread;
//use anyhow::Result;
use slint::LogicalPosition;
//use tray_item::{IconSource, TrayItem};

slint::include_modules!();

fn main() {
    
     let ui= AppWindow::new().unwrap();
    //musicplay("src/assets/songs/Guitar, Loneliness and Blue Planet.flac");
    let handle = ui.as_weak();
    let hanel_close = ui.as_weak();
    ui.on_close_window(move ||{
        hanel_close.upgrade().unwrap().hide().unwrap();
    });
    
    ui.on_music_play(move ||{
        thread::spawn(|| {
            musicplay("src/assets/songs/Guitar, Loneliness and Blue Planet.flac");
        });
        
    });
    
    ui.on_move_window(move |offset_x, offset_y|{
        let main = handle.upgrade().unwrap();
        //获取窗口的物理坐标
        let logical_pos = main.window().position().to_logical(main.window().scale_factor());
        //窗口坐标添加上偏移量，设置为新的位置
        main.window().set_position(LogicalPosition::new(logical_pos.x + offset_x, logical_pos.y + offset_y));
    });
    let _ = ui.run();

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
