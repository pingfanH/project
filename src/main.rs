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
mod structs;
slint::include_modules!();
#[tokio::main]
async fn main() {
    json::perready().await;
    let ui: AppWindow= AppWindow::new().unwrap();
    let musicplayer=Arc::new(musicplayer::MusicPlayer::new());

    let slintsender=Arc::new(slinttrait::SlintSender::new(&ui));

    let hanel_close = ui.as_weak();
    tray_item::tray_item_run(hanel_close,&ui,musicplayer,slintsender).await;

}