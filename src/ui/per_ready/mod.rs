use std::{path::Path, fs, sync::Arc};

use slint::{SharedString, Image, SharedPixelBuffer, Rgba8Pixel};

use crate::{AppWindow,file_lib,slinttrait::SlintSender, env::{APP_PATH, SERVER_URL, AVATAR}};
use crate::ui::call_backs::messages_call_backs;

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

    ui.set_user_avatar(image);
    if let Ok(entries) = fs::read_dir(&format!("{}data/musiclist/",APP_PATH)) {
        let has_files = entries
        .filter_map(Result::ok)
        .any(|entry| entry.file_type().ok().map(|ft| ft.is_file()).unwrap_or(false));        
        if has_files{


        let account=file_lib::read_cookie("account").await.unwrap();
        messages_call_backs::update_my_music_list(SERVER_URL.to_owned(),account,slint_sender.channel.clone()).await;


        messages_call_backs::update_pub_music_list(SERVER_URL.to_owned(),slint_sender.channel.clone()).await;


        };
    };

}