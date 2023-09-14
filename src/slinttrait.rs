use std::{fs::Metadata, path::Path};
use std::sync::Arc;

use crate::{AppWindow,env::{self, AVATAR}};
use futures::future::{Fuse,FutureExt};
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel, Image, SharedPixelBuffer, Rgba8Pixel};
use std::rc::Rc;
use std::str::FromStr;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

#[derive(Debug)]
pub enum SlintMessage {
    Quit,
    SetValue(String,String,Option<Vec<(slint::SharedString, slint::SharedString, bool, slint::SharedString)>>),
    Test(String),
}

#[derive(Debug)]
pub struct SlintSender {
    pub channel: UnboundedSender<SlintMessage>,
    worker_thread: std::thread::JoinHandle<()>,
}

impl SlintSender {
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
    //     let _ = self.channel.send(SlintMessage::Quit);
    //     self.worker_thread.join()
    // }
}

async fn cargo_worker_loop(
    mut r: UnboundedReceiver<SlintMessage>,
    handle: slint::Weak<AppWindow>,
) -> tokio::io::Result<()> {
    let trait_set = Fuse::terminated();

    futures::pin_mut!(
        trait_set,
    );
    loop {
        let m: SlintMessage = futures::select! {
            res = trait_set => {
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
            SlintMessage::Quit => return Ok(()),
            SlintMessage::SetValue(function,Value,List) =>
            trait_set.set(set_value(handle.clone(),function,Value,List).fuse()),
            SlintMessage::Test(string)=>
            {set_playing(handle.clone(),string).fuse().await;return Ok(())},
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
                 "set_avatar"=>{
                    let image = match Image::load_from_path(Path::new(&value)){
                        Ok(image) => image,
                        Err(_) =>{
                            let mut cat_image = image::load_from_memory(AVATAR).expect("err").into_rgba8();
                            image::imageops::colorops::brighten_in_place(&mut cat_image, 20);
                        
                            let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                                cat_image.as_raw(),
                                cat_image.width(),
                                cat_image.height(),
                            );
                            Image::from_rgba8(buffer)
                        },
                    };
                    h.set_user_avatar(image);
                    
                 },
                &_ => todo!(),
            }
            // let code =format!("h.{}(SharedString::from({}))",function, value);
            // h.set_user_name(value.into());
            // let result = eval(&code);
        })
        .unwrap();
}
async fn set_playing(handle: slint::Weak<AppWindow>,value:String){
    handle
        .clone()
        .upgrade_in_event_loop(move |h| {
           println!("{}",value);
           h.set_user_name(SharedString::from(value));
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
