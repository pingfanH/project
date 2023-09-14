use std::sync::Arc;

use crate::env::{APP_PATH, SERVER_URL};
use crate::musicplayer::{MusicPlayer, self};
use crate::{AppWindow, request_lib};
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
        let name=musiclistdata.name;
        let user=musiclistdata.user;
        let date=musiclistdata.date;

        println!("music:{}",name);
        let music=format!("{}data/music/{}{}{}",APP_PATH,user,name,date);
        musicplayer3.channel.send(musicplayer::MusicMessage::ChangeMusic(music)).unwrap();
    });
    let musicplayer4=musicplayer.clone();
    ui.on_tryplay(move|tryplaydata|{  
        let musicplayer4=musicplayer4.clone();
        let url=format!("{}/api/downloapubmusic/{}{}{}",SERVER_URL.to_owned(),tryplaydata.user,tryplaydata.name,tryplaydata.date);
        println!("{url}");
        tokio::spawn(async move{
            let file: Vec<u8> = request_lib::get_file(&url)
            .await
            .unwrap();
        musicplayer4.channel.send(musicplayer::MusicMessage::TryPlay(file)).unwrap();
        });
    });
    let musicplayer5=musicplayer.clone();
    ui.on_setvolume(move|volume|{
        musicplayer5.channel.send(musicplayer::MusicMessage::ChangeVolume(volume)).unwrap();
    });


}