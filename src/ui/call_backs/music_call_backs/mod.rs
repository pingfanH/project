use std::fs::File;
use std::sync::Arc;

use slint::SharedString;

use crate::env::{APP_PATH, SERVER_URL};
use crate::musicplayer::{MusicPlayer, self};
use crate::{AppWindow, request_lib, file_lib};
pub fn main(ui:&AppWindow,musicplayer:Arc<MusicPlayer>){
        //播放音乐
        let musicplayer1=musicplayer.clone();
        ui.on_music_play(move||{
           
            musicplayer1.channel.send(musicplayer::MusicMessage::Play).unwrap();
        });
        let musicplayer2=musicplayer.clone();
        ui.on_music_pause(move||{
            
            musicplayer2.channel.send(musicplayer::MusicMessage::Pause).unwrap();
        });
        let musicplayer3=musicplayer.clone();
    ui.on_choose_music(move|musiclistdata|{
       let id =musiclistdata.id;
        
        println!("music id:{}",id);
        let music=id;
        musicplayer3.channel.send(musicplayer::MusicMessage::ChangeMusic(music.into())).unwrap();
    });
    let musicplayer4=musicplayer.clone();
    ui.on_tryplay(move|tryplaydata|{  
        let musicplayer4=musicplayer4.clone();
        let url=format!("{}api/downloadmusic/{}",SERVER_URL.to_owned(),tryplaydata.id);
        println!("{url}");
        let targe=format!("{}data/music/{}",APP_PATH,tryplaydata.id);
        tokio::spawn(async move{
           match request_lib::get_file(&url).await{
            Ok(file) =>{
                file_lib::writefile(&targe,&file).await;
                musicplayer4.channel.send(musicplayer::MusicMessage::TryPlay(file)).unwrap();
            },
            Err(_) =>eprintln!("get error"),
        }
        });
        // tokio::spawn(async move{
        //     let file: Vec<u8> = request_lib::get_file(&url)
        //     .await
        //     .unwrap();
        // musicplayer4.channel.send(musicplayer::MusicMessage::TryPlay(file)).unwrap();
        // });
    });
    let musicplayer5=musicplayer.clone();
    ui.on_setvolume(move|volume|{
        musicplayer5.channel.send(musicplayer::MusicMessage::ChangeVolume(volume)).unwrap();
    });
    let musicplayer6=musicplayer.clone();
    ui.on_cancel_tryplay(move||{
        musicplayer6.channel.send(musicplayer::MusicMessage::CancelTryPlayer()).unwrap();
    });
    let musicplayer7=musicplayer.clone();
    ui.on_music_last_next(move|last_next|{
        let last_next:String=last_next.to_string();
        println!("{}",last_next);
        musicplayer7.channel.send(musicplayer::MusicMessage::MusicNext(last_next)).unwrap();
    });

}