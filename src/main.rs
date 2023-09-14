#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate serde_json;
use std::io::Cursor;
use std::io::Seek;
use std::io::SeekFrom;
use rodio::Decoder;
use serde_json::json;
use std::sync::Arc;
mod request_lib;
mod json;
mod file_lib;
mod slinttrait;
mod musicplayer;
mod ultis;
mod tray_item;
mod env;
mod ui;
slint::include_modules!();
#[tokio::main]
async fn main() {
    json::perready().await;
    let ui: AppWindow= AppWindow::new().unwrap();
    let mut persound = Cursor::new(env::PERSOUND.to_vec());
    persound.seek(SeekFrom::Start(2)).unwrap();
    let decoder = Decoder::new(persound).unwrap();
    let musicplayer=Arc::new(musicplayer::MusicPlayer::new(decoder));

    let slintsender=Arc::new(slinttrait::SlintSender::new(&ui));

    let hanel_close = ui.as_weak();
    tray_item::tray_item_run(hanel_close,&ui,musicplayer,slintsender).await;

}