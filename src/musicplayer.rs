use std::io::{self, Cursor, SeekFrom, Seek};
use std::ops::Deref;
use std::{thread, fs::File};
use std::fmt::format;
use std::time::Duration;
use futures::io::empty;
use rodio::decoder;
use serde_json::Value;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender, UnboundedReceiver};
use rodio::{Decoder, OutputStream, Sink, source::Source};
use std::sync::{Arc, Mutex};

use crate::env::APP_PATH;
use crate::ultis;

#[derive(Debug)]
pub enum MusicMessage {
    None,
    Play,
    Pause,
    ChangeMusic(String),
    ChangeVolume(i32),
    TryPlay(Vec<u8>),
    CancelTryPlayer(),
    MusicNext(String),
}

pub struct MusicPlayer {
    pub channel: UnboundedSender<MusicMessage>,
    worker_thread: thread::JoinHandle<()>,
}

impl MusicPlayer {
    pub fn new() -> Self {

        let (channel, r) = tokio::sync::mpsc::unbounded_channel();
        let worker_thread = thread::spawn(move || {
            tokio::runtime::Runtime::new().unwrap().block_on(music_player_loop(r)).unwrap();
        });
        Self {
            channel,
            worker_thread,
        }
    }
}

async fn music_player_loop(mut r: UnboundedReceiver<MusicMessage>) -> Result<(), Box<dyn std::error::Error>> {
    // 创建音频输出流
    let (_stream, stream_handle) = OutputStream::try_default()?;


    let sink = Sink::try_new(&stream_handle)?;


    let music_list=crate::file_lib::readfilenameloop(&format!("{}data/musiclist/", crate::env::APP_PATH)).await;
    for id in music_list{
        let name=id;
        let filename=format!("{}data/music/{}", crate::env::APP_PATH,name);
        if let Ok(file)=File::open(&filename){
            println!("append music:{}",&filename);
            let source=Decoder::new(file).unwrap();
            sink.append(source);
        }else{
            println!("can not append music:{}",&filename);
        }
    }
    sink.pause();
    let try_player_sink = Sink::try_new(&stream_handle)?;


   
   let mut music_playing = false;
   let mut current_music = String::new();
   loop {
       let m = match r.recv().await {
           Some(m) => m,
           None =>break,
       };
       match m {
            MusicMessage::Play => {
               if !music_playing {
                println!("Playing music");
                   sink.play();
                   music_playing = true;
               }
           }
           MusicMessage::Pause => {
                if music_playing {
                    println!("Pausing music");
                    sink.pause();
                    music_playing = false;
                    
                }
            }
            MusicMessage::ChangeMusic(path) => {
                
                
                let music_list=crate::file_lib::readfile(&format!("{}data/musiclist/data.json", crate::env::APP_PATH)).await.unwrap();
                let music_list:Vec<Value>=serde_json::from_str(&music_list).unwrap();
                let mut is_find_targe=false;
                println!("music_list:{music_list:#?}");
                sink.stop();
                sink.pause();
                current_music=path.clone();
                let filename=format!("{}data/music/{}", crate::env::APP_PATH,path);
                
                if let Ok(file)=File::open(&filename){
                    let source=Decoder::new(file).unwrap();
                    sink.append(source);
                }
                music_playing=true;
                sink.play();
            }
            MusicMessage::ChangeVolume(new_volume) => {
                let volume=new_volume as f32;

                println!("Changing volume to: {}", volume/50.0);
                sink.set_volume(volume/50.0);
            }
            MusicMessage::TryPlay(file)=>{
                let mut file = Cursor::new(file);
                // 在需要定位的位置调用seek方法
                file.seek(SeekFrom::Start(2)).unwrap();
                let source=Decoder::new(file)?;
                try_player_sink.append(source);
                try_player_sink.play();
            },
            MusicMessage::CancelTryPlayer()=>{
                try_player_sink.stop();
            },
            MusicMessage::MusicNext(last_next)=>{

            },
            MusicMessage::None=>{

            }
        }

    }
    Ok(())
}


