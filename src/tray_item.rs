use std::{sync::{mpsc, Arc}, process};

use slint::{Weak, ComponentHandle};
use tray_item::{TrayItem, IconSource};
use crate::{ultis, ui, musicplayer::{self, MusicPlayer}, slinttrait::SlintSender,env::{APP_PATH}};

use super::AppWindow;


enum Message {
    Quit,
    LogOff,
    Run,
    Play,
    Pause
}

pub async fn tray_item_run(hanel_close:Weak<AppWindow>,ui:&AppWindow,musicplayerarc:Arc<MusicPlayer>,slint_sender_arc:Arc<SlintSender>){
    let mut tray = TrayItem::new(
        "P-layer",
        IconSource::Resource("name-of-icon-in-rc-file"),
    )
    .unwrap();
    tray.add_label("P-layer").unwrap();
    tray.inner_mut().add_separator().unwrap();
    let (tx, rx) = mpsc::sync_channel(1);
    let run_tx = tx.clone();

    tray.add_menu_item("启动", move || {
        run_tx.send(Message::Run).unwrap(); 
    })
    .unwrap();

    let logoff_tx = tx.clone();
    tray.add_menu_item("退出登录", move || {
        logoff_tx.send(Message::LogOff).unwrap();
    })
    .unwrap();

    let play_tx = tx.clone();
    tray.add_menu_item("播放音乐", move || {
        play_tx.send(Message::Play).unwrap();
    })
    .unwrap();
    
    let pause_tx = tx.clone();
    tray.add_menu_item("暂停音乐", move || {
        pause_tx.send(Message::Pause).unwrap();
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
                Ok(Message::LogOff) => {
                    println!("Logoff");
                    ultis::delete_folder_contents(&format!("{}cookies",APP_PATH));
                    hanel_close.upgrade().unwrap().hide().unwrap();
                    let cargoworkerarc=slint_sender_arc.clone();
                    let musicplayerarc=musicplayerarc.clone();
                    ui::apprun(&ui,musicplayerarc,cargoworkerarc).await;  
                }
                Ok(Message::Play) => {
                    musicplayerarc.channel.send(musicplayer::MusicMessage::Play);
                }
                Ok(Message::Pause) => {
                    musicplayerarc.channel.send(musicplayer::MusicMessage::Pause);
                }
                Ok(Message::Run) => {
                    let cargoworkerarc=slint_sender_arc.clone();
                    let musicplayerarc=musicplayerarc.clone();
                    ui::apprun(&ui,musicplayerarc,cargoworkerarc).await;  
                }
                _ => {}
            }
        }
    
}
