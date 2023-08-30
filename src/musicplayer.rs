use std::io::{self, Cursor, SeekFrom, Seek};
use std::ops::Deref;
use std::{thread, fs::File};
use std::time::Duration;
use rodio::decoder;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender, UnboundedReceiver};
use rodio::{Decoder, OutputStream, Sink, source::Source};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum MusicMessage {
    Quit,
    Play,
    Pause,
    ChangeMusic(String),
    ChangeVolume(u32),
    TryPlay(Vec<u8>)
}

pub struct MusicPlayer {
    pub channel: UnboundedSender<MusicMessage>,
    worker_thread: thread::JoinHandle<()>,
    current_source: Option<Arc<Mutex<Decoder<File>>>>,
}

impl MusicPlayer {
    pub fn new(decoder:Decoder<File>) -> Self {
        let (channel, r) = tokio::sync::mpsc::unbounded_channel();
        let worker_thread = thread::spawn(move || {
            tokio::runtime::Runtime::new().unwrap().block_on(music_player_loop(r,decoder)).unwrap();
        });
        Self {
            channel,
            worker_thread,
            current_source: None, // 初始化为None
        }
    }
}

async fn music_player_loop(mut r: UnboundedReceiver<MusicMessage>,mut decoder:Decoder<File>) -> Result<(), Box<dyn std::error::Error>> {
    // 创建音频输出流
    let (_stream, stream_handle) = OutputStream::try_default()?;


    let sink = Sink::try_new(&stream_handle)?;
    //sink.append(decoder);

    let source = Arc::new(decoder); // 使用Arc包装source
    
    let share_source:Decoder<File> =match Arc::try_unwrap(source){
        Ok(decoder) => decoder,
        Err(err) => todo!(),
    };

    sink.append(share_source);
    sink.pause();
    //let share_source=source.clone();

    
    // 创建音频播放器);
    

   
   let mut music_playing = false;
   let mut current_music = String::new();
   let mut volume = 50;

   loop {
       let m = match r.recv().await {
           Some(m) => m,
           None => break,
       };

       match m {
            MusicMessage::Quit => break,
            MusicMessage::Play => {
               if !music_playing {
                println!("Playing music: {}", current_music);
                   sink.play();
                   music_playing = true;
               }
           }
           MusicMessage::Pause => {
                if music_playing {
                    println!("Pausing music: {}", current_music);
                    sink.pause();
                    music_playing = false;
                    
                }
            }
            MusicMessage::ChangeMusic(path) => {
                println!("Changing music to: {}", path);
                if let Ok(file) = File::open(&path) {
                    if let Ok(new_source) = Decoder::new(file) {
                        
                        sink.stop();
                        sink.pause();
                        sink.append(new_source);
                        sink.play();
                        music_playing=true;
                    } else {
                        println!("无法解码音频文件！");
                    }
                } else {
                    println!("无法打开音频文件！");
                }
            }
            MusicMessage::ChangeVolume(new_volume) => {
                println!("Changing volume to: {}", new_volume);
                volume = new_volume;
            }
            MusicMessage::TryPlay(file)=>{
                sink.stop();
                sink.pause();
                let mut file = Cursor::new(file);

                // 在需要定位的位置调用seek方法
                file.seek(SeekFrom::Start(2)).unwrap();
                let source = Decoder::new(file).unwrap();
                sink.append(source);
                sink.play();
                music_playing=true;
            }
        }
    }

    Ok(())
}

// async fn playstreammusic(file:Vec<u8>){
//     let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    
//     let sink = Sink::try_new(&stream_handle).unwrap();
//     sink.append(source);
//     sink.sleep_until_end();
// }