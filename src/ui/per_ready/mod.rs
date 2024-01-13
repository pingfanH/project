use std::{path::Path, fs, sync::Arc};

use serde_json::Value;
use slint::{SharedString, Image, SharedPixelBuffer, Rgba8Pixel, ModelRc, VecModel};

use crate::{AppWindow,file_lib,slinttrait::SlintSender, env::{APP_PATH, SERVER_URL, AVATAR}, request_lib};
use crate::ui::call_backs::messages_call_backs;
use crate::PlayList;
pub async fn main(ui:&AppWindow,slint_sender:Arc<SlintSender>){
    let cookie=match file_lib::readfile(&format!("{}cookies/.token",APP_PATH)).await{
        Ok(ok)=>{
            ui.set_is_login(true);
            ui.set_user_name(SharedString::from(file_lib::read_cookie("name").await.unwrap()));
            ui.set_user_id(SharedString::from(file_lib::read_cookie("id").await.unwrap()));
            ui.set_user_age(SharedString::from(file_lib::read_cookie("age").await.unwrap()));
            ui.set_user_intro(SharedString::from(file_lib::read_cookie("intro").await.unwrap()));
            ui.set_user_gender(SharedString::from(file_lib::read_cookie("gender").await.unwrap()));
            ui.set_user_sign_date(SharedString::from(file_lib::read_cookie("sign_date").await.unwrap()));
            ui.set_user_music_number(file_lib::read_cookie("music_number").await.unwrap().parse().unwrap());
            ok
        },
        Err(_)=>"err".to_owned(),
    };
    
    println!("{}",cookie);

    let image = match Image::load_from_path(Path::new(&format!("{}data/user-avatar/avatar.png",APP_PATH))){
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
    println!("set avatar");
    ui.set_user_avatar(image);
    if let Ok(entries) = fs::read_dir(&format!("{}data/musiclist/",APP_PATH)) {
        let has_files = entries
        .filter_map(Result::ok)
        .any(|entry| entry.file_type().ok().map(|ft| ft.is_file()).unwrap_or(false));        
        if has_files{
        messages_call_backs::update_my_music_list(slint_sender.channel.clone()).await;
        messages_call_backs::update_pub_music_list(slint_sender.channel.clone()).await;
        };
    };
    let account=file_lib::read_cookie("account").await.unwrap();
    let url=&format!("{}api/get_play_list/{}/all",SERVER_URL,account);
    match request_lib::get(url).await{
        Ok(play_lists) =>{
            let play_lists:Value=serde_json::from_str(&play_lists).unwrap();
            //println!("play_lists:{play_lists:#?}");
            let mut vec_play_list:Vec<PlayList>=vec![];
            let play_lists_array=play_lists.as_array().unwrap();
            for play_list in play_lists_array{
                //println!("play_list:{play_list:#?}");
                let name=play_list["name"].to_string();
                let user=play_list["user"].to_string();
                let intro=play_list["intro"].to_string();
                let id: String=play_list["id"].to_string();
                let date=play_list["date"].to_string();
                vec_play_list.push(
                    PlayList{
                        date: date[1..date.len()-1].to_owned().into(),
                        id: id[1..id.len()-1].to_owned().into(),
                        intro: intro[1..intro.len()-1].to_owned().into(),
                        title: name[1..name.len()-1].to_owned().into(),
                        user:  user[1..user.len()-1].to_owned().into(),
                    }
                )
            }
            let play_list:Vec<Vec<PlayList>>=
            vec_play_list
            .chunks(2)
            .map(|chunk| {
                let vec1:PlayList=PlayList{title:chunk[0].title.clone().into(),user:chunk[0].user.clone().into(),date:chunk[0].date.clone().into(),id:chunk[0].id.clone().into(),intro:chunk[0].intro.clone().into()};
                if chunk.len()>1{
                    let vec2=PlayList{title:chunk[1].title.clone().into(),user:chunk[1].user.clone().into(),date:chunk[1].date.clone().into(),id:chunk[1].id.clone().into(),intro:chunk[1].intro.clone().into()};
                    vec![vec1,vec2]
                }
                else{
                    vec![vec1]
                }
            }
            )
            .collect();
        let mut play_lists:Vec<(ModelRc<PlayList>,)>=vec![];
        for cplay_list in play_list{
            let cpla:ModelRc<PlayList>=ModelRc::new(VecModel::from(cplay_list));
            let cpla:(ModelRc<PlayList>,)=(cpla,);
            play_lists.push(cpla);
        }
            let vec_play_list:ModelRc<(ModelRc<PlayList>,)>=ModelRc::new(VecModel::from(play_lists));
           ui.set_playlistcards(vec_play_list);
        },
        Err(_) =>{println!("network failed")},
    }
}