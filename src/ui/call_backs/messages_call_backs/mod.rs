use rfd::FileDialog;
use serde_json::json;

use crate::{AppWindow, request_lib,file_lib, structs::MusicList, slinttrait::{SlintMessage, SlintSender}};
use std::{sync::Arc, collections::HashMap, fs::File};
use crate::env::{APP_PATH, SERVER_URL};
use crate::CardList;
pub fn main(ui:&AppWindow,slint_sender:Arc<SlintSender>){
let slintsendrclone=slint_sender.clone();
//上传音乐
ui.on_musicupload(move|name,is_public|{
        let files = FileDialog::new()
        .add_filter("music", &["mp3","mp4","flac","wav","ogg","avi","mov","mkv"])
        //.add_filter("rust", &["rs", "toml"])
        .set_directory("/")
        .pick_file();
    print!("name{}",name.as_str());
    if let Some(files ) = files{
            let path=files.display().to_string();
            // let path=path.as_str().replace("\\", "/");
            println!("{}",path);
            let slint_sender_channel = slintsendrclone.channel.clone();
            tokio::spawn(async move{
                match  file_lib::read_cookie("account").await{
                    Ok(ok)=>{
                        
                        let name=name.as_str();
                        let filename=path.split(r"\").last().unwrap();
                        let index = filename.rfind('.').unwrap();
                        let filename = &filename[..index];
                        let filename=filename.replace(",", "");
                        println!("{}",name);
                        let url=format!("{}api/uploadmusic/{}",SERVER_URL,is_public);
                        request_lib::upload(path.as_str(),url.as_str(), ok.as_str(),filename.as_str()).await;

                        let filejsonname=filename.clone();
                        //thread::sleep(Duration::from_secs(1));
                        let url=format!("{}api/getmusic/{}/{}/all",SERVER_URL,&ok,filename);
                        println!("geturl{}",url);
                        let filename=match request_lib::get(url.as_str()).await{
                            Ok(ok)=>ok,
                            Err(err)=>"err".to_string(),
                        };
                        let filename=format!("{}data/musiclist/{}",APP_PATH,filename);
                        file_lib::writefile(&filename,b"").await;
                        let jsonstr1=match request_lib::get(&format!("{}api/getmusiclist/{}/all",SERVER_URL,ok)).await{
                            Ok(ok)=>ok,
                            Err(err)=>todo!(),
                        };

                        let filejson=match request_lib::get(&format!("{}api/getmusic/{}/{}/all",SERVER_URL,ok,filejsonname)).await{
                            Ok(ok)=>ok,
                            Err(err)=>todo!(),
                        };
                        
                        println!("copypath:{}",path.as_str());
                        let musicdate:Vec<_>=filejson.split(",").collect();
                        let filejsonuser=musicdate[0];
                        let filejsonname=musicdate[1];
                        let filejsondate=musicdate[2];

                        file_lib::copyfile(path.as_str(),&format!("{}data/music/{}{}{}",APP_PATH,filejsonuser,filejsonname,filejsondate)).await.unwrap();


                        

                        let musicList: Vec<MusicList> = serde_json::from_str(jsonstr1.as_str()).unwrap();
        println!("{:?}",musicList);
        
        //let mut musiclist1=musicList.clone();
        let mut musiclist2: Vec<(slint::SharedString, slint::SharedString, bool, slint::SharedString)> = Vec::new();
        for list in musicList{
            println!("{:?}",list);
            musiclist2.push((list.date.clone().into(),list.name.clone().into(),list.public.clone(),list.user.clone().into()));
            let path=format!("{}data/musiclist/{},{},{},{}",APP_PATH,<std::string::String as Into<String>>::into(list.user.clone().into()),<std::string::String as Into<String>>::into(list.name.clone().into()),list.date,list.public);
            file_lib::writefile(path.as_str(), b"").await;
        };
        println!("{:?}",musiclist2);
        // let mut musiclistrc1: ModelRc<(slint::SharedString, bool, slint::SharedString)> =
        // ModelRc::new(VecModel::from(musiclist2));

        
        slint_sender_channel.send(SlintMessage::SetList("set_musiclist".to_string(),musiclist2)).unwrap();
    
    

        let publicjsonstr=request_lib::get(format!("{}api/getmusiclistall/true",SERVER_URL).as_str()).await.unwrap();
        //println!("{}",jsonstr);
        let publicmusiclist: Vec<MusicList> = serde_json::from_str(publicjsonstr.as_str()).unwrap();
        println!("{:?}",publicmusiclist);
        let mut publicmusicList: Vec<(slint::SharedString, slint::SharedString, bool, slint::SharedString)> = Vec::new();
        for list in publicmusiclist{
            println!("{:?}",list);
            publicmusicList.push((list.date.clone().into(),list.name.clone().into(),list.public.clone(),list.user.clone().into()));
            let path=format!("{}data/publicmusicList/{},{},{}",APP_PATH,<std::string::String as Into<String>>::into(list.user.clone().into()),<std::string::String as Into<String>>::into(list.name.clone().into()),list.date);
            file_lib::writefile(path.as_str(), b"").await;
        };

        slint_sender_channel.send(SlintMessage::SetList("set_publicmusic".to_string(),publicmusicList)).unwrap();
        //println!("musiclistrc{:#?}",musiclistrc);
    //ui.set_publicmusic(pubmusiclistrc);


    },
                    Err(err)=>eprintln!("{}",err)
                }
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
        tokio::spawn(async move{
            let account=file_lib::read_cookie("account").await.unwrap();
            request_lib::upload(&path1,&format!("{}api/upload",SERVER_URL),&account, "avatar.png").await;

            file_lib::copyfile(path1.as_str(), &format!("{}data/user-avatar/avatar.png",APP_PATH)).await;
        });
        slint_sender_channel.send(SlintMessage::SetAvatar(path)).unwrap();
    };

});
//刷新列表
let slintsendrclone=slint_sender.clone();
ui.on_refresh(move||{
    let slint_sender_channel = slintsendrclone.channel.clone();
    tokio::spawn(async move{
    let publicjsonstr=request_lib::get(&format!("{}api/getmusiclistall/true",SERVER_URL)).await.unwrap();
    let publicmusiclist: Vec<MusicList> = serde_json::from_str(publicjsonstr.as_str()).unwrap();
    println!("{:?}",publicmusiclist);
    let mut publicmusic_list: Vec<(slint::SharedString, slint::SharedString, bool, slint::SharedString)> = Vec::new();
    for list in publicmusiclist{
        println!("{:?}",list);
        publicmusic_list.push((list.date.clone().into(),list.name.clone().into(),list.public.clone(),list.user.clone().into()));
        let path=format!("{}data/publicmusicList/{},{},{}",APP_PATH,<std::string::String as Into<String>>::into(list.user.clone().into()),<std::string::String as Into<String>>::into(list.name.clone().into()),list.date);
        file_lib::writefile(path.as_str(), b"").await;
    };

    slint_sender_channel.send(SlintMessage::SetList("set_publicmusic".to_string(),publicmusic_list)).unwrap();
    });
});
//收藏音乐
let slintsendrclone=slint_sender.clone();
ui.on_collect(move|collectdata|{
    let url=format!("{}/api/downloapubmusic",SERVER_URL);
    println!("{url}");
    let pubmuslist=format!("{}data/musiclist/{},{},{},{}",APP_PATH,collectdata.user,collectdata.name,collectdata.date,"true");
   
    let slint_sender_channel = slintsendrclone.channel.clone();
    tokio::spawn(async move{
        file_lib::writefile(pubmuslist.as_str(), b"").await.unwrap();
        let path=format!("{}data/music/{}{}{}",APP_PATH,collectdata.user,collectdata.name,collectdata.date);
        let name: String=format!("{}{}{}",collectdata.user,collectdata.name,collectdata.date);
        request_lib::downloadpubmusic(&url,&name, &path).await;


        let useraccount =file_lib::read_cookie("account").await.unwrap();

        update_my_music_list(SERVER_URL.to_owned(),useraccount,slint_sender_channel).await;

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
        request_lib::download(&format!("{}api/download",SERVER_URL),&account.as_str(),"avatar.png",&format!("{}data/user-avatar/avatar.png",APP_PATH)).await;
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
        request_lib::upload(avatarpath.clone(),&format!("{}",SERVER_URL),&account, "avatar.png").await.unwrap();
        file_lib::copyfile(avatarpath.clone(), &format!("{}data/user-avatar/avatar.png",APP_PATH)).await.unwrap();
        
        slint_sender_channel.send(SlintMessage::SetValue("set_avatar".to_string(),avatarpath.clone().to_owned())).unwrap();

    });
   
});
}

pub async fn update_my_music_list(server_url:String,useraccount:String,cargo_worker:tokio::sync::mpsc::UnboundedSender<SlintMessage>){
    let jsonstr1=match request_lib::get(&format!("{}api/getmusiclist/{}/all",server_url,useraccount)).await{
        Ok(ok)=>ok,
        Err(err)=>todo!(),
    };
    let music_list: Vec<MusicList> = match serde_json::from_str(jsonstr1.as_str()){
        Ok(music_list) => {println!("{:?}",music_list);music_list},
        Err(err) => {eprintln!("{}",err);vec![]},
    };
    
    
    //let mut musiclist1=musicList.clone();
    let mut musiclist2: Vec<(slint::SharedString, slint::SharedString, bool, slint::SharedString)> = Vec::new();
    for list in music_list{
        println!("{:?}",list);
        musiclist2.push((list.date.clone().into(),list.name.clone().into(),list.public.clone(),list.user.clone().into()));
        let path=format!("{}data/musiclist/{},{},{},{}",APP_PATH,<std::string::String as Into<String>>::into(list.user.clone().into()),<std::string::String as Into<String>>::into(list.name.clone().into()),list.date,list.public);
        file_lib::writefile(path.as_str(), b"").await;
    };
    println!("{:?}",musiclist2);
    
    cargo_worker.send(SlintMessage::SetList("set_musiclist".to_string(),musiclist2)).unwrap();


}

pub async fn update_pub_music_list(
    server_url:String,
    cargo_worker:tokio::sync::mpsc::UnboundedSender<SlintMessage>
)
{
    let publicjsonstr=request_lib::get(format!("{}api/getmusiclistall/true",server_url).as_str()).await.unwrap();

    let publicmusiclist: Vec<MusicList> = match serde_json::from_str(publicjsonstr.as_str()){
        Ok(music_list) => {println!("{:?}",music_list);music_list},
        Err(err) => {eprintln!("{}",err);vec![]},
    };
    let pubmusiclist:Vec<Vec<crate::CardList>>=
    publicmusiclist
    .chunks(2)
    .map(|chunk| {
        let vec1:CardList=CardList{music:chunk[0].name.clone().into(),user:chunk[0].user.clone().into(),date:chunk[0].date.clone().into()};
        if chunk.len()>1{
            let vec2=CardList{music:chunk[1].name.clone().into(),user:chunk[1].user.clone().into(),date:chunk[1].date.clone().into()};
            vec![vec1,vec2]
        }
        else{
            vec![vec1]
        }
    }
    )
    .collect();
    println!("{:#?}",pubmusiclist);
    cargo_worker.send(SlintMessage::SetPubList(pubmusiclist)).unwrap();
}

