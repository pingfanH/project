use std::fs::Metadata;

use super::{AppWindow};
use anyhow::Context;

use futures::future::{Fuse, FusedFuture, FutureExt};
//use itertools::Itertools;
use serde::Deserialize;
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};
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
    SetValue(String,String)
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

    futures::pin_mut!(
        trait_set,
    );
    loop {
        let m: CargoMessage = futures::select! {
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
            CargoMessage::Quit => return Ok(()),
            CargoMessage::SetValue(function,Value) => {
                trait_set.set(set_value(handle.clone(),function,Value).fuse());
            }
    }
}
}
async fn set_value(handle: slint::Weak<AppWindow>,function:String,value:String){
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
                "set_user_music_number" => {
                    match value.parse::<i32>() {
                        Ok(parsed_value) => h.set_user_music_number(parsed_value),
                        Err(err) => {
                            eprintln!("Failed to parse value: {}", err);
                        }
                    }
                },
                "set_avatar_path" => h.set_avatar_path(value.into()),

                &_ => todo!(),
            }
            // let code =format!("h.{}(SharedString::from({}))",function, value);
            // h.set_user_name(value.into());
            // let result = eval(&code);
        })
        .unwrap();
}
