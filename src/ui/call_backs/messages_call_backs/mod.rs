use image::EncodableLayout;
use reqwest::Client;
use rfd::FileDialog;
use serde_json::{json, Value};
use ncm_parser;
use crate::{AppWindow, request_lib,file_lib, structs::MusicList, slinttrait::{SlintMessage, SlintSender}};
use std::{sync::Arc, collections::HashMap, fs::File};
use crate::env::{APP_PATH, SERVER_URL};
use crate::CardList;
use crate::env;
pub fn main(ui:&AppWindow,slint_sender:Arc<SlintSender>){
let slintsendrclone=slint_sender.clone();
//上传音乐
ui.on_musicupload(move|name,is_public|{
        let files = FileDialog::new()
        .add_filter("music", &["mp3","mp4","flac","wav","ogg","avi","mov","mkv","ncm"])
        //.add_filter("rust", &["rs", "toml"])
        .set_directory("/")
        .pick_file();
    print!("name{}",name.as_str());
    if let Some(files) = files{
            let path=files.display().to_string();
            // let path=path.as_str().replace("\\", "/");
            println!("{}",path);
            let slint_sender_channel = slintsendrclone.channel.clone();
            tokio::spawn(async move{
                let account=file_lib::read_cookie("account").await.unwrap();
                let name=name.as_str();
                let filename=path.split(r"\").last().unwrap();
                let extension=path.split(r".").last().unwrap();
                let index = filename.rfind('.').unwrap();
                let mut filename = &filename[..index];
                if name!=""{
                    filename=name;
                };
                println!("{}",name);
                let url=format!("{}api/uploadmusic/{}/{}/{}",SERVER_URL,is_public,account.as_str(),filename);
                //判断后缀
                match extension{
                    "ncm"=>{
                        let ncm_file = std::fs::read(&path).unwrap();
                        let mut ncm_file_from_iter =
                        ncm_parser::from_iter(ncm_file.into_iter()).unwrap();
                        let music = ncm_file_from_iter.get_music().unwrap();
                        let temp=&format!("{}ncm.temp",APP_PATH);
                        std::fs::write(temp, &music).unwrap();
                        let id: String=request_lib::upload(temp,url.as_str()).await.unwrap();
                        file_lib::copyfile(&temp.as_str(),&format!("{}data/music/{}",APP_PATH,id)).await.unwrap();
                    },
                    &_=>{
                        let id=request_lib::upload(path.as_str(),url.as_str()).await.unwrap();
                        file_lib::copyfile(path.as_str(),&format!("{}data/music/{}",APP_PATH,id)).await.unwrap();
                    },
                }
                update_my_music_list(slint_sender_channel.clone()).await;
                update_pub_music_list(slint_sender_channel.clone()).await;
            });
    }
    
    }
    
);
//选择头像
let slintsendrclone=slint_sender.clone();
ui.on_choose_avatar(move||{
    let slint_sender_channel = slintsendrclone.channel.clone();
    let files = FileDialog::new()
    .add_filter("music", &["png"])
    .set_directory("/")
    .pick_file();
if let Some(files ) = files{
        let path=files.display().to_string();
        println!("{}",path);
        let path1=path.clone();
        slint_sender_channel.send(SlintMessage::SetAvatar(path));
        tokio::spawn(async move{
            let account=file_lib::read_cookie("account").await.unwrap();
            request_lib::upload(&path1,&format!("{}api/upload/avatar/{}",SERVER_URL,&account)).await;

            file_lib::copyfile(path1.as_str(), &format!("{}data/user-avatar/avatar.png",APP_PATH)).await;
        }); 
    };

});
//刷新列表
let slintsendrclone=slint_sender.clone();
ui.on_refresh(move||{
    let slint_sender_channel = slintsendrclone.channel.clone();
    tokio::spawn(async move{
        update_pub_music_list(slint_sender_channel).await;
    });
});
//收藏音乐
let slintsendrclone=slint_sender.clone();
ui.on_collect(move|collectdata|{
    let url=format!("{}/api/downloadmusic",SERVER_URL);
    println!("{url}");

    let slint_sender_channel = slintsendrclone.channel.clone();
    tokio::spawn(async move{
        let json_data=file_lib::readfile(&format!("{}data/musiclist/data.json",APP_PATH)).await.unwrap();
        let mut jsondata:Vec<Value>=serde_json::from_str(&json_data).unwrap();
        jsondata.push(json!({
            "data":*collectdata.date.clone(),
            "user":*collectdata.user.clone(),
            "id":*collectdata.id.clone(),
            "name":*collectdata.name.clone(),
            "public":collectdata.public,
        }));
        let jsondata=serde_json::to_string(&jsondata).unwrap();
        file_lib::writefile(&format!("{}data/musiclist/data.json",APP_PATH),jsondata.as_bytes());
        let id=collectdata.id.clone().to_string();
        let path=format!("{}data/music/{}",APP_PATH,id);
        update_my_music_list(slint_sender_channel).await;
        request_lib::downloadpubmusic(&url,&id, &path).await;
    });
});
//应用个人信息更改
ui.on_apply(move |updated_user_data|{
    tokio::spawn(async move {

        let token=match file_lib::readfile(&format!("{}cookies/.token",APP_PATH)).await{
            Ok(ok)=>{
                ok
            },
            Err(_)=>"err".to_owned(),
        };
        let mut user_data = HashMap::new();
        user_data.insert("user_token", token.as_str());
        user_data.insert("user_name", updated_user_data.name.as_str());
        user_data.insert("user_age", updated_user_data.age.as_str());
        user_data.insert("user_intro", &updated_user_data.intro.as_str());
        user_data.insert("user_gender", &updated_user_data.gender.as_str());
        
        let user_data = vec![json!(user_data)];
        let body = serde_json::to_string(&user_data).unwrap();
        println!("apply {:#?}", body);
            let name = updated_user_data.name;
            let age = updated_user_data.age.clone();
            let intro = updated_user_data.intro.clone();
            let gender = updated_user_data.gender.clone();

            
        file_lib::creat_cookie("name", name.as_bytes()).await;
        file_lib::creat_cookie("age", age.as_bytes()).await;
        file_lib::creat_cookie("intro", intro.as_bytes()).await;
        file_lib::creat_cookie("gender", gender.as_bytes()).await;

        request_lib::post(&format!("{}api/user/update",SERVER_URL), body.as_str()).await;
        


        
    });
});
let slintsendrclone=slint_sender.clone();
//登录
ui.on_login( move|account,password|{
    let slint_sender_channel = slintsendrclone.channel.clone();
    tokio::spawn(async move {
        //ui.set_user_name(SharedString::from("123"));
        let account:&str=account.as_str();
        let password:&str=password.as_str();
        //println!("{},{}",account,password);
        let url=format!("{}api/get_token/{}/{}",SERVER_URL,account,password);
        let user_data=request_lib::get(url.as_str()).await.unwrap();
        println!("{}",user_data);

        let user_data=crate::json::str2json(user_data.as_str());
        
        slint_sender_channel.send(SlintMessage::SetValue("set_user_name".to_string(),user_data["user_name"].to_string().replace('"',""))).unwrap();
        slint_sender_channel.send(SlintMessage::SetValue("set_user_age".to_string(),user_data["user_age"].to_string().replace('"',""))).unwrap();
        slint_sender_channel.send(SlintMessage::SetValue("set_user_id".to_string(),user_data["user_id"].to_string().replace('"',""))).unwrap();
        slint_sender_channel.send(SlintMessage::SetValue("set_user_gender".to_string(),user_data["user_gender"].to_string().replace('"',""))).unwrap();
        slint_sender_channel.send(SlintMessage::SetValue("set_user_intro".to_string(),user_data["user_intro"].to_string().replace('"',""))).unwrap();
        slint_sender_channel.send(SlintMessage::SetValue("set_user_music_number".to_string(),user_data["user_music_number"].to_string().replace('"',""))).unwrap();
        slint_sender_channel.send(SlintMessage::SetValue("set_user_sign_date".to_string(),user_data["user_sign_date"].to_string().replace('"',""))).unwrap();
        
        //let mut name = Arc::new(user_data["user_name"].to_string().clone());
        file_lib::creat_cookie("token", user_data["user_token"].to_string().replace('"',"").as_bytes()).await;
        file_lib::creat_cookie("account", user_data["user_account"].to_string().replace('"',"").as_bytes()).await;
        file_lib::creat_cookie("password", user_data["user_password"].to_string().replace('"',"").as_bytes()).await;
        file_lib::creat_cookie("id", user_data["user_id"].to_string().replace('"',"").as_bytes()).await;
        file_lib::creat_cookie("name", user_data["user_name"].to_string().replace('"',"").as_bytes()).await;
        file_lib::creat_cookie("age", user_data["user_age"].to_string().replace('"',"").as_bytes()).await;
        file_lib::creat_cookie("intro", user_data["user_intro"].to_string().replace('"',"").as_bytes()).await;
        file_lib::creat_cookie("gender", user_data["user_gender"].to_string().replace('"',"").as_bytes()).await;
        file_lib::creat_cookie("sign_date", user_data["user_sign_date"].to_string().replace('"',"").as_bytes()).await;
        file_lib::creat_cookie("music_number", user_data["user_music_number"].to_string().replace('"',"").as_bytes()).await;


        let account=file_lib::read_cookie("account").await.unwrap();
        request_lib::download(&format!("{}api/download/avatar/{}",SERVER_URL,account),&format!("{}data/user-avatar/avatar.png",APP_PATH)).await;
        slint_sender_channel.send(SlintMessage::SetValue("set_avatar".to_string(),format!("{}data/user-avatar/avatar.png",APP_PATH).clone().to_owned())).unwrap();
        



       
    });

});
//注册
let slintsendrclone=slint_sender.clone();
ui.on_sign(move|sign_data|{
    let slint_sender_channel = slintsendrclone.channel.clone();
    
    let body=format!("{},{},{},{},{},{}", sign_data.account.as_str(),sign_data.password.as_str(),sign_data.name.as_str(),sign_data.gender.as_str(),sign_data.age.as_str(),sign_data.intro.as_str());
    //let user_name=SignData.name;

    tokio::spawn(async move {
        request_lib::post(&format!("{}api/user/create",SERVER_URL),body.as_str()).await;
        file_lib::creat_cookie("account", sign_data.account.as_bytes()).await;
        file_lib::creat_cookie("password", sign_data.password.as_bytes()).await;
        file_lib::creat_cookie("name", sign_data.name.as_bytes()).await;
        file_lib::creat_cookie("gender", sign_data.gender.as_bytes()).await;
        file_lib::creat_cookie("age", sign_data.age.as_bytes()).await;
        file_lib::creat_cookie("intro", sign_data.intro.as_bytes()).await;
        let avatarpath=sign_data.avatarpath.as_str();
        let account=sign_data.account.as_str();

        println!("");
        request_lib::upload(avatarpath.clone(),&format!("{}api/upload/avatar/{}",SERVER_URL,&account)).await.unwrap();
        file_lib::copyfile(avatarpath.clone(), &format!("{}data/user-avatar/avatar.png",APP_PATH)).await.unwrap();
        
        slint_sender_channel.send(SlintMessage::SetValue("set_avatar".to_string(),avatarpath.clone().to_owned())).unwrap();

    });
   
});
// 创建歌单
ui.on_new_play_list(move|name,intro,public|{
    println!("new play list:{},{},{}",name,intro,public);
    tokio::spawn(async move{
        let account=file_lib::read_cookie("account").await.unwrap();
        let data=serde_json::json!({
            "user":account,
            "name":name.to_string(),
            "intro":intro.to_string(),
            "public":public
        });
        println!("{}",data);
        request_lib::post(&format!("{}api/new_play_list",SERVER_URL),serde_json::to_string(&data).unwrap().as_str()).await;
    });
    
});
//进入歌单
let slintsendrclone=slint_sender.clone();
ui.on_init_play_list(move|id|{
    let slint_sender_channel = slintsendrclone.channel.clone();
    println!("play_list_id:{}",id);
    tokio::spawn(async move{
        let content=request_lib::get(&format!("{}api/get_play_list_content/{}",SERVER_URL,id)).await.unwrap();
        let content_json:Vec<serde_json::Value>=serde_json::from_str(&content).unwrap();
        let mut music_contents:Vec<crate::musiclistdata>=vec![];
        for music_content in content_json{
            let date=music_content["date"].to_string();
            let id=music_content["id"].to_string();
            let name=music_content["name"].to_string();
            let user=music_content["user"].to_string();

            let music_content=crate::musiclistdata{
                date:date[1..date.len()-1].into(),
                id:id[1..id.len()-1].into(),
                name:name[1..name.len()-1].into(),
                public:music_content["public"].as_bool().unwrap(),
                user:user[1..user.len()-1].into(),
            };
            music_contents.push(music_content);
        }
        let _=slint_sender_channel.send(SlintMessage::SetPlayList(music_contents));
    });
   
});
let slintsendrclone=slint_sender.clone();
ui.on_init_collect(move||{
    let slint_sender_channel = slintsendrclone.channel.clone();
    tokio::spawn(async move{
        let account=file_lib::read_cookie("account").await.unwrap();
        let music_list=request_lib::get(&format!("{}api/getmusiclist/{}/all",SERVER_URL,account)).await.unwrap();
        let music_list_json:Vec<serde_json::Value>=serde_json::from_str(&music_list).unwrap();
        let mut music_contents:Vec<crate::musiclistdata>=vec![];
        for music_content in music_list_json{
            let date=music_content["date"].to_string();
            let id=music_content["id"].to_string();
            let name=music_content["name"].to_string();
            let user=music_content["user"].to_string();

            let music_content=crate::musiclistdata{
                date:date[1..date.len()-1].into(),
                id:id[1..id.len()-1].into(),
                name:name[1..name.len()-1].into(),
                public:music_content["public"].as_bool().unwrap(),
                user:user[1..user.len()-1].into(),
            };
            music_contents.push(music_content);
        }
        let _=slint_sender_channel.send(SlintMessage::SetPlayList(music_contents));
    });
});
}
pub async fn update_my_music_list(cargo_worker:tokio::sync::mpsc::UnboundedSender<SlintMessage>){
    let server_url=env::SERVER_URL;
    let useraccount =file_lib::read_cookie("account").await.unwrap();
    let jsonstr1=match request_lib::get(&format!("{}api/getmusiclist/{}/all",server_url,useraccount)).await{
        Ok(ok)=>ok,
        Err(err)=>"err".to_owned(),
    };
    let music_list: Vec<MusicList> = match serde_json::from_str(jsonstr1.as_str()){
        Ok(music_list) => {println!("{:?}",music_list);music_list},
        Err(err) => {eprintln!("{}",err);vec![]},
    };
    let mut json_data:Vec<serde_json::Value>=vec![];

    let mut musiclist2: Vec<(slint::SharedString, slint::SharedString, slint::SharedString, bool, slint::SharedString)> = Vec::new();
    for list in music_list{
        println!("{:?}",list);
        musiclist2.push((list.date.clone().into(),list.id.clone().into(),list.name.clone().into(),list.public.clone(),list.user.clone().into()));
        //let path=format!("{}data/musiclist/{},{},{},{}",APP_PATH,<std::string::String as Into<String>>::into(list.user.clone().into()),<std::string::String as Into<String>>::into(list.name.clone().into()),list.date,list.public);
        json_data.push(json!({
            "name":list.name,
            "date":list.date,
            "user":list.user,
            "public":list.public,
            "id":list.id
        }))
    };
    println!("{:?}",json_data);
    let content=serde_json::to_string(&json_data).unwrap();
    file_lib::writefile(&format!("{}data/musiclist/data.json",APP_PATH,),content.as_bytes()).await;
    
    cargo_worker.send(SlintMessage::SetList("set_musiclist".to_string(),musiclist2));


}
pub async fn update_pub_music_list(
    cargo_worker:tokio::sync::mpsc::UnboundedSender<SlintMessage>
)
{
    let server_url=env::SERVER_URL;
    let publicjsonstr=request_lib::get(format!("{}api/getmusiclistall/true",server_url).as_str()).await.unwrap();

    let publicmusiclist: Vec<MusicList> = match serde_json::from_str(publicjsonstr.as_str()){
        Ok(music_list) => {println!("music_list {:?}",music_list);music_list},
        Err(err) => {eprintln!("{}",err);vec![]},
    };
    let pubmusiclist:Vec<Vec<crate::CardList>>=
    publicmusiclist
    .chunks(2)
    .map(|chunk| {
        let vec1:CardList=CardList{music:chunk[0].name.clone().into(),user:chunk[0].user.clone().into(),date:chunk[0].date.clone().into(),id:chunk[0].id.clone().into()};
        if chunk.len()>1{
            let vec2=CardList{music:chunk[1].name.clone().into(),user:chunk[1].user.clone().into(),date:chunk[1].date.clone().into(),id:chunk[1].id.clone().into()};
            vec![vec1,vec2]
        }
        else{
            vec![vec1]
        }
    }
    )
    .collect();
    println!("{:#?}",pubmusiclist);
    cargo_worker.send(SlintMessage::SetPubList(pubmusiclist));
}

