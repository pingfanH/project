use std::fs::Metadata;
use std::sync::Arc;

use super::{AppWindow};
use anyhow::Context;

use futures::future::{Fuse, FusedFuture, FutureExt};
//use itertools::Itertools;
use serde::Deserialize;
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel, Image, SharedPixelBuffer, Rgba8Pixel};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::str::FromStr;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use eval::eval;
#[derive(Debug)]
pub struct FeatureSettings {
    enabled_features: Vec<SharedString>,
    enable_default_features: bool,
}

#[derive(Debug)]
pub enum CargoMessage {
    Quit,
    SetValue(String,String,Option<Vec<(slint::SharedString, slint::SharedString, bool, slint::SharedString)>>),
    SetPlaying(bool),
    SetAvatar(String)
}

pub struct CargoWorker {
    pub channel: UnboundedSender<CargoMessage>,
    worker_thread: std::thread::JoinHandle<()>,
}

impl CargoWorker {
    pub fn new(cargo_ui: &AppWindow) -> Self {
        let (channel, r) = tokio::sync::mpsc::unbounded_channel();
        let worker_thread = std::thread::spawn({
            let handle_weak = cargo_ui.as_weak();
            move || {
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(cargo_worker_loop(r, handle_weak))
                    .unwrap()
            }
        });
        Self {
            channel,
            worker_thread,
        }
    }

    // pub fn join(self) -> std::thread::Result<()> {
    //     let _ = self.channel.send(CargoMessage::Quit);
    //     self.worker_thread.join()
    // }
}

async fn cargo_worker_loop(
    mut r: UnboundedReceiver<CargoMessage>,
    handle: slint::Weak<AppWindow>,
) -> tokio::io::Result<()> {
    let trait_set = Fuse::terminated();
    let trait_set1 = Fuse::terminated();

    futures::pin_mut!(
        trait_set,
        trait_set1,
    );
    loop {
        let m: CargoMessage = futures::select! {
            res = trait_set => {
                res;
                continue;
            }
            res = trait_set1 => {
                res;
                continue;
            }
            m = r.recv().fuse() => {
                match m {
                    None => return Ok(()),
                    Some(m) => m,
                }
            }
        };

        match m {
            CargoMessage::Quit => return Ok(()),
            CargoMessage::SetValue(function,Value,List) => {
                trait_set.set(set_value(handle.clone(),function,Value,List).fuse());
            },
            CargoMessage::SetPlaying(Value) => {
                //trait_set1.set(set_playing(handle.clone(),Value).fuse())
            },
            CargoMessage::SetAvatar(avatarpath) => {
                trait_set1.set(set_avatar(handle.clone(),avatarpath).fuse());
            }
    }
}
fn parse_string_to_bool(string: &str) -> Option<bool> {
    match bool::from_str(string) {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}
}
async fn set_value(handle: slint::Weak<AppWindow>,function:String,value:String,list:Option<Vec<(slint::SharedString, slint::SharedString, bool, slint::SharedString)>>){
    let mut model: Arc<Rc<dyn Model<Data = (SharedString,SharedString, bool, SharedString)>>>;
    
    
    handle
        .clone()
        .upgrade_in_event_loop(move |h| {
            
            match function.as_str(){
                "set_user_age" => h.set_user_age(value.into()),
                "set_user_id" => h.set_user_id(value.into()),
                "set_user_name" => h.set_user_name(value.into()),
                "set_user_gender" => h.set_user_gender(value.into()),
                "set_user_sign_date" => h.set_user_sign_date(value.into()),
                "set_user_intro" => h.set_user_intro(value.into()),
                "set_is_playing" => {
                    h.set_is_playing(str2bool(&value));
                }
                "on_music_play" => {
                    h.on_music_play(||{});
                }
                "set_user_music_number" => {
                    match value.parse::<i32>() {
                        Ok(parsed_value) => h.set_user_music_number(parsed_value),
                        Err(err) => {
                            eprintln!("Failed to parse value: {}", err);
                        }
                    }
                },
                "set_avatar_path" => h.set_avatar_path(value.into()),
                "set_musiclist" => {
                   let list:Vec<(slint::SharedString, slint::SharedString, bool, slint::SharedString)>= match list {
                        Some(list) =>list,
                        None => todo!(),
                    };
                    println!("list{:?}",list.clone());
                    let mut musiclistrc1: ModelRc<(slint::SharedString, slint::SharedString, bool, slint::SharedString)> =
                        ModelRc::new(VecModel::from(list));
                    h.set_musiclist(musiclistrc1);
                },
                "set_publicmusic" => {
                    let list:Vec<(slint::SharedString, slint::SharedString, bool, slint::SharedString)>= match list {
                         Some(list) =>list,
                         None => todo!(),
                     };
                     println!("list{:?}",list.clone());
                     let mut musiclistrc1: ModelRc<(slint::SharedString, slint::SharedString, bool, slint::SharedString)> =
                         ModelRc::new(VecModel::from(list));
                     h.set_publicmusic(musiclistrc1);
                 },
                &_ => todo!(),
            }
            // let code =format!("h.{}(SharedString::from({}))",function, value);
            // h.set_user_name(value.into());
            // let result = eval(&code);
        })
        .unwrap();
}
async fn set_playing(handle: slint::Weak<AppWindow>,value:bool){
    handle
        .clone()
        .upgrade_in_event_loop(move |h| {
            h.set_is_playing(value);
        })
        .unwrap();
}
async fn set_avatar(handle: slint::Weak<AppWindow>,avaterpath:String){
    handle
        .clone()
        .upgrade_in_event_loop(move |h| {
            let mut cat_image = image::open(avaterpath).expect("Error loading cat image").into_rgba8();

            image::imageops::colorops::brighten_in_place(&mut cat_image, 20);
    
            let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                cat_image.as_raw(),
                cat_image.width(),
                cat_image.height(),
            );
            let image = Image::from_rgba8(buffer);

            h.set_user_avatar(image);
        })
        .unwrap();
}

pub fn str2bool(s: &str) ->bool {
    match s.to_lowercase().as_str() {
        "true" | "t" | "yes" | "y" | "1" => true,
        "false" | "f" | "no" | "n" | "0" => false,
        _ => todo!(),
    }
}
