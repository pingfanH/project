#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate serde_json;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Cursor;
use std::io::Seek;
use std::io::SeekFrom;
use std::path;
use std::path::Path;
use std::pin::Pin;
use std::process;
use std::rc::Rc;
use std::sync::Mutex;
use std::time::Duration;
use musicplayer::MusicPlayer;
use rodio::{Decoder, OutputStream, Sink};
use slint::Image;
use slint::ModelRc;
use slint::Rgba8Pixel;
use slint::SharedPixelBuffer;
use slint::VecModel;
use slinttrait::CargoWorker;
use std::thread;
use slint::LogicalPosition;
use slint::SharedString;
use std::collections::HashMap;
use std::fs;
use rfd::FileDialog;
use std::sync::mpsc;
use tray_item::{IconSource, TrayItem};
mod RequestLib;
mod json;
mod FileLib;
mod slinttrait;
mod musicplayer;
use serde_json::json;
use std::sync::{Arc, RwLock};
use std::cell::Cell;
use std::sync::mpsc::{channel, Receiver, Sender};
use tokio::task;
use crate::slinttrait::CargoMessage;
slint::include_modules!();

#[derive(Debug)]
struct User {
    user_token:String,
    user_id: String,
    user_account: String,
    user_password: String,
    user_name: String,
    user_gender: String,
    user_age: String,
    user_intro: String,
    user_sign_date: String,
    user_music_number: i32,
}
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct MusicList {
    user: String,
    name: String,
    date: String,
    public: bool,

}
enum Message {
    Quit,
    LogOff,
    Run,
    Test,
    Test1
}
const PERSOUND: &[u8] = include_bytes!("assets/audios/per");
const CLICKSOUND1: &[u8] = include_bytes!("assets/audios/keypress.mp3");
const CLICKSOUND2: &[u8] = include_bytes!("assets/audios/notes.mp3");
const AVATAR: &[u8] = include_bytes!("assets/icon/avatar.png");

#[tokio::main]
async fn main() {
    env::set_var("SERVER_URL", "http://127.0.0.1:8000/");
    env::set_var("APP_PATH", "C:/Program Files/P-layer/");

    let SERVER_URL = env::var("SERVER_URL").expect("SERVER_URL not found in .env file");
    let APP_PATH = env::var("APP_PATH").expect("APP_PATH not found in .env file");
    perready().await;
    
    let ui: AppWindow= AppWindow::new().unwrap();

    // let cargo_worker = Arc::new(slinttrait::CargoWorker::new(&ui));
    // let cargoworkerarc = cargo_worker.clone();

    let cargo_worker = slinttrait::CargoWorker::new(&ui);
    let filename=format!("{}data/music/per",APP_PATH);
    println!("{filename}");
    let mut file = Cursor::new(PERSOUND.to_vec());
    file.seek(SeekFrom::Start(2)).unwrap();
   // let source = Decoder::new(file).unwrap();
    //let file = File::open(filename).unwrap();
    let decoder = Decoder::new(file).unwrap();

    let musicplayer=musicplayer::MusicPlayer::new(decoder);
    let musicplayerclannel=musicplayer.channel.clone();

    // let musicplayer = Arc::new(musicplayer::MusicPlayer::new(decoder));
    // let musicplayerclannel=musicplayer.channel.clone();
     let musicplayerarc=Arc::new(musicplayer);
     let cargoarc=Arc::new(cargo_worker);

    // let cargo_channel = cargo_worker.channel.clone();
    // let cargoarc=Arc::new(cargo_channel);

    // let musicplayerarc1=musicplayerarc.clone();
    // let musicplayerarc1=match Arc::try_unwrap(musicplayerarc1){
    //     Ok(ok)=>ok,
    //     Err(err)=>todo!("errrrrr"),
    // };
    // let cargoarc1=cargoarc.clone();
    // let cargoarc1=match Arc::try_unwrap(cargoarc1){
    //     Ok(ok)=>ok,
    //     Err(err)=>todo!("errrrrr"),
    // };

   
   
    


    let hanel_close = ui.as_weak();
        let mut tray = TrayItem::new(
            "P-layer",
            IconSource::Resource("name-of-icon-in-rc-file"),
        )
        .unwrap();
    hanel_close.upgrade().unwrap().hide().unwrap();
        tray.add_label("P-layer").unwrap();
        
    
        tray.inner_mut().add_separator().unwrap();
    
        //let musicplayer = Arc::new(musicplayer);

        let (tx, rx) = mpsc::sync_channel(1);
        let run_tx = tx.clone();
        tray.add_menu_item("启动", move || {
            run_tx.send(Message::Run).unwrap();
            //let musicplayer =musicplayer.clone();


               
        })
        .unwrap();

        let logoff_tx = tx.clone();
        tray.add_menu_item("退出登录", move || {
            logoff_tx.send(Message::LogOff).unwrap();
        })
        .unwrap();

        let test_tx = tx.clone();
        tray.add_menu_item("播放音乐", move || {
            test_tx.send(Message::Test).unwrap();
        })
        .unwrap();

    let test1_tx = tx.clone();
    tray.add_menu_item("暂停音乐", move || {
        test1_tx.send(Message::Test1).unwrap();
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
                        match delete_folder_contents("C:/Program Files/P-layer/cookies") {
                            Ok(()) => println!("del"),
                            Err(e) => println!("del err:{}", e),
                        }
                        hanel_close.upgrade().unwrap().hide().unwrap();
                        let _ = ui.run();
                    }
                    Ok(Message::Test) => {
                        musicplayerclannel.send(musicplayer::MusicMessage::Play).unwrap();
                            //thread::sleep(Duration::from_secs(4));
                            
                        //cargo_channel.send(CargoMessage::SetPlaying(true)).unwrap();
                    }
                    Ok(Message::Test1) => {
                        musicplayerclannel.send(musicplayer::MusicMessage::Pause).unwrap();
                        //cargo_channel.send(CargoMessage::SetPlaying(false)).unwrap();
                    }
                    Ok(Message::Run) => {
                        let cargoworkerarc=cargoarc.clone();
                        let musicplayerarc=musicplayerarc.clone();
                        // let _cargoarc=Arc::try_unwrap(cargoworkerarc);
                        // match  _cargoarc{
                        //     Ok(ok)=>print!("success"),
                        //     Err(err)=>print!("faild"),
                        // }
                        // let musicplayerarc=match Arc::try_unwrap(musicplayerarc.clone()){
                        //     Ok(ok)=>ok,
                        //     Err(err)=>todo!(""),
                        // };
                        // Apprun(&ui,musicplayerarc,cargoarc).await;
                        Apprun(&ui,musicplayerarc,cargoworkerarc).await;
                       
                        //let _ = ui.run();

                        
                    }
                    _ => {}
                }
            }



}

async fn Apprun(ui: &AppWindow, musicplayer:Arc<MusicPlayer>,cargo_worker:Arc<CargoWorker>) {

    let musicplayerarc1=musicplayer.clone();
    let cargoarc1=cargo_worker.clone();
    env::set_var("SERVER_URL", "http://127.0.0.1:8000/");
    env::set_var("APP_PATH", "C:/Program Files/P-layer/");
    let SERVER_URL = env::var("SERVER_URL").expect("SERVER_URL not found in .env file");
    let APP_PATH = env::var("APP_PATH").expect("APP_PATH not found in .env file");

    let handle = ui.as_weak();
    let hanel_close = ui.as_weak();
    
    let musicplayer=musicplayerarc1.clone();
    
    let mut istoken:bool=false;

    let cookie=match FileLib::readfile("C:/Program Files/P-layer/cookies/.token").await{
        Ok(ok)=>{istoken=true;ok},
        Err(err)=>{istoken=false;"err"}.to_owned(),
    };
    
    println!("{}",cookie);

    if(istoken){

                    ui.set_is_login(true);
                    match FileLib::read_cookie("name").await{
                        Ok(ok)=>{ui.set_user_name(SharedString::from(ok))},
                        Err(err)=>eprintln!("{}",err)
                    }
                    match FileLib::read_cookie("id").await{
                        Ok(ok)=>ui.set_user_id(SharedString::from(ok)),
                        Err(err)=>eprintln!("{}",err)
                    }
                    match FileLib::read_cookie("age").await{
                        Ok(ok)=>ui.set_user_age(SharedString::from(ok)),
                        Err(err)=>eprintln!("{}",err)
                    }
                    match FileLib::read_cookie("intro").await{
                        Ok(ok)=>ui.set_user_intro(SharedString::from(ok)),
                        Err(err)=>eprintln!("{}",err)
                    }
                    match FileLib::read_cookie("gender").await{
                        Ok(ok)=>ui.set_user_gender(SharedString::from(ok)),
                        Err(err)=>eprintln!("{}",err)
                    }
                    match FileLib::read_cookie("sign_date").await{
                        Ok(ok)=>ui.set_user_sign_date(SharedString::from(ok)),
                        Err(err)=>eprintln!("{}",err)
                    }
                    match FileLib::read_cookie("music_number").await{
                        Ok(ok)=>ui.set_user_music_number(ok.parse().unwrap()),
                        Err(err)=>eprintln!("{}",err)
                    }
        };

    let mut cat_image = image::load_from_memory(AVATAR).expect("err").into_rgba8();

    image::imageops::colorops::brighten_in_place(&mut cat_image, 20);

    let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
        cat_image.as_raw(),
        cat_image.width(),
        cat_image.height(),
    );
    let image = Image::from_rgba8(buffer);
    ui.set_user_avatar(image);
    if let Ok(entries) = fs::read_dir(&format!("C:/Program Files/P-layer/data/musiclist/")) {
        let has_files = entries
        .filter_map(Result::ok)
        .any(|entry| entry.file_type().ok().map(|ft| ft.is_file()).unwrap_or(false));        
        if has_files{


    let account=FileLib::read_cookie("account").await.unwrap();
    let cargo_channel = cargoarc1.channel.clone();
    let cargo_channe2 = cargoarc1.channel.clone();
    let SERVER_URL1=SERVER_URL.clone();
    let SERVER_URL2=SERVER_URL.clone();
    update_my_music_list(SERVER_URL1,account,cargo_channel).await;


    update_pub_music_list(SERVER_URL2,cargo_channe2).await;


    };
    };

    //监听按键
    // thread::spawn(|| {
    //     let device_state = DeviceState::new();
    //     let keys: Vec<Keycode> = device_state.get_keys();
    //     loop{
    //         //if keys.contains(&Keycode::A)
    //         //{

    //             println!("{}",keys.contains(&Keycode::A));
    //         //}
    //     }
    // });
    
    //关闭窗口

    ui.on_close_window(move ||{
        musicplay(CLICKSOUND2);
        hanel_close.upgrade().unwrap().hide().unwrap();
    });
    //跳转到url
    ui.on_open_the_url(|url|{
        println!("{}",url); 
        open::that(url.as_str()).ok();
    });
    //播放点击音效1,2
    ui.on_click_audio(move ||{
        thread::spawn(|| {
            musicplay(CLICKSOUND1);
        });
    });

    ui.on_click_audio1(move ||{
        thread::spawn(|| {
            musicplay(CLICKSOUND2);
        });
    });
    ui.on_test(move|value|{
        //let cargo_channel = cargo_worker.channel.clone();
        
        
        //let message=cargo_channel.send(CargoMessage::GetValues()).unwrap();
       //cargo_channel.send(CargoMessage::SetValue("on_music_play".to_string(),"true".to_string())).unwrap();
        

    });
    let cargo_worker = Arc::new(slinttrait::CargoWorker::new(&ui));
    let cargo_channel = cargo_worker.channel.clone();
    let cargo_channe2 = cargo_worker.channel.clone();

    let SERVER_URLclone=SERVER_URL.clone();
    let SERVER_URLclone1=SERVER_URLclone.clone();
    let SERVER_URLclone2=SERVER_URLclone.clone();
    let SERVER_URLclone3=SERVER_URLclone.clone();
    let APP_PATHclone=APP_PATH.clone();
    let cargo_channel2 = cargo_channe2.clone();
    
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
            let cargo_channel = cargo_channel.clone();
            let SERVER_URL=SERVER_URLclone.clone();
            let APP_PATH=APP_PATH.clone();
            tokio::spawn(async move{
                match  FileLib::read_cookie("account").await{
                    Ok(ok)=>{
                        
                        let name=name.as_str();
                        let filename=path.split(r"\").last().unwrap();
                        let index = filename.rfind('.').unwrap();
                        let filename = &filename[..index];
                        let filename=filename.replace(",", "");
                        println!("{}",name);
                        let url=format!("{}api/uploadmusic/{}",SERVER_URL,is_public);
                        RequestLib::upload(path.as_str(),url.as_str(), ok.as_str(),filename.as_str()).await;

                        let filejsonname=filename.clone();
                        //thread::sleep(Duration::from_secs(1));
                        let url=format!("{}api/getmusic/{}/{}/all",SERVER_URL,&ok,filename);
                        println!("geturl{}",url);
                        let filename=match RequestLib::get(url.as_str()).await{
                            Ok(ok)=>ok,
                            Err(err)=>"err".to_string(),
                        };
                        let filename=format!("C:/Program Files/P-layer/data/musiclist/{}",filename);
                        FileLib::writefile(&filename,b"").await;
                        let jsonstr1=match RequestLib::get(&format!("{}api/getmusiclist/{}/all",SERVER_URL,ok)).await{
                            Ok(ok)=>ok,
                            Err(err)=>todo!(),
                        };

                        let filejson=match RequestLib::get(&format!("{}api/getmusic/{}/{}/all",SERVER_URL,ok,filejsonname)).await{
                            Ok(ok)=>ok,
                            Err(err)=>todo!(),
                        };
                        
                        println!("copypath:{}",path.as_str());
                        let musicdate:Vec<_>=filejson.split(",").collect();
                        let filejsonuser=musicdate[0];
                        let filejsonname=musicdate[1];
                        let filejsondate=musicdate[2];

                        FileLib::copyfile(path.as_str(),&format!("C:/Program Files/P-layer/data/music/{}{}{}",filejsonuser,filejsonname,filejsondate)).await.unwrap();


                        

                        let musicList: Vec<MusicList> = serde_json::from_str(jsonstr1.as_str()).unwrap();
        println!("{:?}",musicList);
        
        //let mut musiclist1=musicList.clone();
        let mut musiclist2: Vec<(slint::SharedString, slint::SharedString, bool, slint::SharedString)> = Vec::new();
        for list in musicList{
            println!("{:?}",list);
            musiclist2.push((list.date.clone().into(),list.name.clone().into(),list.public.clone(),list.user.clone().into()));
            let path=format!("C:/Program Files/P-layer/data/musiclist/{},{},{},{}",<std::string::String as Into<String>>::into(list.user.clone().into()),<std::string::String as Into<String>>::into(list.name.clone().into()),list.date,list.public);
            FileLib::writefile(path.as_str(), b"").await;
        };
        println!("{:?}",musiclist2);
        // let mut musiclistrc1: ModelRc<(slint::SharedString, bool, slint::SharedString)> =
        // ModelRc::new(VecModel::from(musiclist2));

        
        cargo_channel.send(CargoMessage::SetValue("set_musiclist".to_string(),"1".to_string(),Some(musiclist2))).unwrap();
    
    

        let publicjsonstr=RequestLib::get(format!("{}api/getmusiclistall/true",SERVER_URL).as_str()).await.unwrap();
        //println!("{}",jsonstr);
        let publicmusiclist: Vec<MusicList> = serde_json::from_str(publicjsonstr.as_str()).unwrap();
        println!("{:?}",publicmusiclist);
        let mut publicmusicList: Vec<(slint::SharedString, slint::SharedString, bool, slint::SharedString)> = Vec::new();
        for list in publicmusiclist{
            println!("{:?}",list);
            publicmusicList.push((list.date.clone().into(),list.name.clone().into(),list.public.clone(),list.user.clone().into()));
            let path=format!("C:/Program Files/P-layer/data/publicmusicList/{},{},{}",<std::string::String as Into<String>>::into(list.user.clone().into()),<std::string::String as Into<String>>::into(list.name.clone().into()),list.date);
            FileLib::writefile(path.as_str(), b"").await;
        };

        cargo_channel.send(CargoMessage::SetValue("set_publicmusic".to_string(),"1".to_string(),Some(publicmusicList))).unwrap();
        //println!("musiclistrc{:#?}",musiclistrc);
    //ui.set_publicmusic(pubmusiclistrc);


    },
                    Err(err)=>eprintln!("{}",err)
                }
            });

    }
    
    }
    
);


    ui.on_refresh(move||{
        let cargo_channel3 = cargo_channel2.clone();
        let SERVER_URL2=SERVER_URLclone3.clone();
        tokio::spawn(async move{
            let publicjsonstr=RequestLib::get(format!("{}api/getmusiclistall/true",SERVER_URL2).as_str()).await.unwrap();
        //println!("{}",jsonstr);
        let publicmusiclist: Vec<MusicList> = serde_json::from_str(publicjsonstr.as_str()).unwrap();
        println!("{:?}",publicmusiclist);
        let mut publicmusicList: Vec<(slint::SharedString, slint::SharedString, bool, slint::SharedString)> = Vec::new();
        for list in publicmusiclist{
            println!("{:?}",list);
            publicmusicList.push((list.date.clone().into(),list.name.clone().into(),list.public.clone(),list.user.clone().into()));
            let path=format!("C:/Program Files/P-layer/data/publicmusicList/{},{},{}",<std::string::String as Into<String>>::into(list.user.clone().into()),<std::string::String as Into<String>>::into(list.name.clone().into()),list.date);
            FileLib::writefile(path.as_str(), b"").await;
        };

        cargo_channel3.send(CargoMessage::SetValue("set_publicmusic".to_string(),"1".to_string(),Some(publicmusicList))).unwrap();
        });
    });

    let cargo_worker = Arc::new(slinttrait::CargoWorker::new(&ui));
    let cargo_channel = cargo_worker.channel.clone();
    let cargo_channe2 = cargo_worker.channel.clone();

    ui.on_choose_avatar(move||{
        let cargo_channel = cargo_channel.clone();
        let files = FileDialog::new()
        .add_filter("music", &["png"])
        .set_directory("/")
        .pick_file();
    if let Some(files ) = files{
            let path=files.display().to_string();
            // let path=path.as_str().replace("\\", "/");
            println!("{}",path);
            let cargo_channel = cargo_worker.channel.clone();
            cargo_channel.send(CargoMessage::SetValue("set_avatar_path".to_string(),path,None)).unwrap();

        };
       


    });
    let musicplayerchannel=musicplayer.channel.clone();
    let musicplayerchannel2=musicplayerchannel.clone();

    ui.on_choose_music(move|musiclistdata:musiclistdata|{
        let name=musiclistdata.name;
        let user=musiclistdata.user;
        let date=musiclistdata.date;

        println!("music:{}",name);
        let music=format!("C:/Program Files/P-layer/data/music/{}{}{}",user,name,date);
        musicplayerchannel.send(musicplayer::MusicMessage::ChangeMusic(music)).unwrap();
    });
    let SERVER_URL=SERVER_URL.clone();
    let SERVER_URL1=SERVER_URL.clone();
    let SERVER_URL3=SERVER_URL.clone();
    let APPPATH=APP_PATHclone.clone();
    let musicplayerchannel3=musicplayerchannel2.clone();
    let musicplayerchannel6=musicplayerchannel2.clone();
    
    ui.on_tryplay(move|tryplaydata|{  
        let musicplayerchannel3=musicplayerchannel3.clone();
        let url=format!("{}/api/downloapubmusic/{}{}{}",SERVER_URL1,tryplaydata.user,tryplaydata.name,tryplaydata.date);
        println!("{url}");
        tokio::spawn(async move{

            let file: Vec<u8> = RequestLib::get_file(&url)
            .await
            .unwrap();
        musicplayerchannel3.send(musicplayer::MusicMessage::TryPlay(file)).unwrap();
        });

    });

    ui.on_collect(move|collectdata|{
        let url=format!("{}/api/downloapubmusic",SERVER_URL3);
        println!("{url}");
        let APPPATH=APPPATH.clone();
        let SERVER_URL=SERVER_URL3.clone();
        let pubmuslist=format!("C:/Program Files/P-layer/data/musiclist/{},{},{},{}",collectdata.user,collectdata.name,collectdata.date,"true");
       
        let cargo_channel1 = cargo_channe2.clone();
        tokio::spawn(async move{
            FileLib::writefile(pubmuslist.as_str(), b"").await;
            let path=format!("{}data/music/{}{}{}",APPPATH,collectdata.user,collectdata.name,collectdata.date);
            let name: String=format!("{}{}{}",collectdata.user,collectdata.name,collectdata.date);
            RequestLib::downloadpubmusic(&url,&name, &path);


        let useraccount =FileLib::read_cookie("account").await.unwrap();

        update_my_music_list(SERVER_URL,useraccount,cargo_channel1).await;

        });
    });
    ui.on_apply( move |updated_user_data|{
        
        let mut user_data = HashMap::new();
        user_data.insert("user_token", cookie.as_str());
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
            let SERVER_URL=SERVER_URL.clone();
        tokio::spawn(async move {
           
            FileLib::creat_cookie("name", name.as_bytes()).await;
            FileLib::creat_cookie("age", age.as_bytes()).await;
            FileLib::creat_cookie("intro", intro.as_bytes()).await;
            FileLib::creat_cookie("gender", gender.as_bytes()).await;
            RequestLib::post(&format!("{}api/user/update",SERVER_URL), body.as_str()).await;
        });
    });
    
    let cargo_worker = Arc::new(slinttrait::CargoWorker::new(&ui));
    let cargo_channel = cargo_worker.channel.clone();
    let cargo_channe4 = cargo_worker.channel.clone();
    ui.on_login( move|account,password|{
    
       
        let cargo_channel = cargo_channel.clone();
        let SERVER_URL=SERVER_URLclone1.clone();
        let APP_PATH=APP_PATHclone.clone();
        
        tokio::spawn(async move {
            //ui.set_user_name(SharedString::from("123"));
            let account:&str=account.as_str();
            let password:&str=password.as_str();
            //println!("{},{}",account,password);
            let mut user_data=format!("{}api/get_token/{}/{}",SERVER_URL,account,password);
            match  RequestLib::get(user_data.as_str()).await{
                Ok(ok)=>{
                    user_data=ok;
                },
                Err(err)=>eprint!("{}",err)
            } 
            println!("{}",user_data);

            let mut user_data=json::str2json(user_data.as_str());
            
            cargo_channel.send(CargoMessage::SetValue("set_user_name".to_string(),user_data["user_name"].to_string().replace('"',""),None)).unwrap();
            cargo_channel.send(CargoMessage::SetValue("set_user_age".to_string(),user_data["user_age"].to_string().replace('"',""),None)).unwrap();
            cargo_channel.send(CargoMessage::SetValue("set_user_id".to_string(),user_data["user_id"].to_string().replace('"',""),None)).unwrap();
            cargo_channel.send(CargoMessage::SetValue("set_user_gender".to_string(),user_data["user_gender"].to_string().replace('"',""),None)).unwrap();
            cargo_channel.send(CargoMessage::SetValue("set_user_intro".to_string(),user_data["user_intro"].to_string().replace('"',""),None)).unwrap();
            cargo_channel.send(CargoMessage::SetValue("set_user_music_number".to_string(),user_data["user_music_number"].to_string().replace('"',""),None)).unwrap();
            cargo_channel.send(CargoMessage::SetValue("set_user_sign_date".to_string(),user_data["user_sign_date"].to_string().replace('"',""),None)).unwrap();
            
            //let mut name = Arc::new(user_data["user_name"].to_string().clone());
            FileLib::creat_cookie("token", user_data["user_token"].to_string().replace('"',"").as_bytes()).await;
            FileLib::creat_cookie("account", user_data["user_account"].to_string().replace('"',"").as_bytes()).await;
            FileLib::creat_cookie("password", user_data["user_password"].to_string().replace('"',"").as_bytes()).await;
            FileLib::creat_cookie("id", user_data["user_id"].to_string().replace('"',"").as_bytes()).await;
            FileLib::creat_cookie("name", user_data["user_name"].to_string().replace('"',"").as_bytes()).await;
            FileLib::creat_cookie("age", user_data["user_age"].to_string().replace('"',"").as_bytes()).await;
            FileLib::creat_cookie("intro", user_data["user_intro"].to_string().replace('"',"").as_bytes()).await;
            FileLib::creat_cookie("gender", user_data["user_gender"].to_string().replace('"',"").as_bytes()).await;
            FileLib::creat_cookie("sign_date", user_data["user_sign_date"].to_string().replace('"',"").as_bytes()).await;
            FileLib::creat_cookie("music_number", user_data["user_music_number"].to_string().replace('"',"").as_bytes()).await;
    

            let account=FileLib::read_cookie("account").await.unwrap();
            RequestLib::download(&format!("{}api/download",SERVER_URL),&account.as_str(),"avatar.png",&format!("{}data/user-avatar/avatar.png",APP_PATH)).await;
            cargo_channel.send(CargoMessage::SetAvatar(format!("{}data/user-avatar/avatar.png",APP_PATH))).unwrap();
           
        });
    
    });

    ui.on_sign(move|sign_data|{
        let cargo_channel = cargo_channe4.clone();
        
        let body=format!("{},{},{},{},{},{}", sign_data.account.as_str(),sign_data.password.as_str(),sign_data.name.as_str(),sign_data.gender.as_str(),sign_data.age.as_str(),sign_data.intro.as_str());
        //let user_name=SignData.name;
        let SERVER_URL=SERVER_URLclone2.clone();

        tokio::spawn(async move {
            RequestLib::post(&format!("{}api/user/create",SERVER_URL),body.as_str()).await;
            FileLib::creat_cookie("account", sign_data.account.as_bytes()).await;
            FileLib::creat_cookie("password", sign_data.password.as_bytes()).await;
            FileLib::creat_cookie("name", sign_data.name.as_bytes()).await;
            FileLib::creat_cookie("gender", sign_data.gender.as_bytes()).await;
            FileLib::creat_cookie("age", sign_data.age.as_bytes()).await;
            FileLib::creat_cookie("intro", sign_data.intro.as_bytes()).await;
            let avatarpath=sign_data.avatarpath.as_str();
            let account=sign_data.account.as_str();


            RequestLib::upload(avatarpath.clone(),&format!("{}",SERVER_URL),&account, "avatar.png").await.unwrap();
            FileLib::copyfile(avatarpath.clone(), "C:/Program Files/P-layer/data/user-avatar/avatar.png").await.unwrap();
            
            cargo_channel.send(CargoMessage::SetAvatar(avatarpath.clone().to_string())).unwrap();
            //ui.set_user_avatar(image);
            
            
        });
       
    });
    //播放音乐

    ui.on_music_play(move||{
        let musicplayerchannel=musicplayer.channel.clone();
        musicplayerchannel.send(musicplayer::MusicMessage::Play).unwrap();
    });
   
    ui.on_music_pause(move||{
        let musicplayerchannel=musicplayerchannel2.clone();
        musicplayerchannel.send(musicplayer::MusicMessage::Pause).unwrap();
    });
    ui.on_setvolume(move|volume|{

        let musicplayerchannel=musicplayerchannel6.clone();
        musicplayerchannel.send(musicplayer::MusicMessage::ChangeVolume(volume)).unwrap();
    });
    ui.on_move_window(move |offset_x, offset_y|{
        let main = handle.upgrade().unwrap();
        //获取窗口的物理坐标
        let logical_pos = main.window().position().to_logical(main.window().scale_factor());
        //窗口坐标添加上偏移量，设置为新的位置
        main.window().set_position(LogicalPosition::new(logical_pos.x + offset_x, logical_pos.y + offset_y));
       
    });
    
    let _ = ui.run();
}

fn delete_folder_contents(folder_path: &str) -> std::io::Result<()> {
    let path = Path::new(folder_path);

    // 遍历文件夹内的内容
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        // 如果是文件，则直接删除
        if entry_path.is_file() {
            fs::remove_file(entry_path)?;
        }
        // 如果是文件夹，则递归调用自身进行删除
        else if entry_path.is_dir() {
            delete_folder_contents(entry_path.to_str().unwrap())?;
            fs::remove_dir(entry_path)?;
        }
    }

    Ok(())
}
fn musicplay(stream:&[u8]){
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    
    //let file = BufReader::new(File::open(path).unwrap());
    let mut file = Cursor::new(stream.to_vec());
    file.seek(SeekFrom::Start(2)).unwrap();
    let source = Decoder::new(file).unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}
async fn perready(){
    env::set_var("SERVER_URL", "http://127.0.0.1:8000/");
    env::set_var("APP_PATH", "C:/Program Files/P-layer/");

    let SERVER_URL = env::var("SERVER_URL").expect("SERVER_URL not found in .env file");

    match fs::create_dir("C:/Program Files/P-layer"){
        Ok(ok)=>ok,
        Err(err)=>eprintln!(""),
    };
    match fs::create_dir("C:/Program Files/P-layer/cookies"){
        Ok(ok)=>ok,
        Err(err)=>eprintln!(""),
    };
    match fs::create_dir("C:/Program Files/P-layer/data"){
        Ok(ok)=>ok,
        Err(err)=>eprintln!(""),
    };
    match fs::create_dir("C:/Program Files/P-layer/data/audios"){
        Ok(ok)=>ok,
        Err(err)=>eprintln!(""),
    };
    match fs::create_dir("C:/Program Files/P-layer/data/music"){
        Ok(ok)=>ok,
        Err(err)=>eprintln!(""),
    };
    match fs::create_dir("C:/Program Files/P-layer/data/musiclist"){
        Ok(ok)=>ok,
        Err(err)=>eprintln!(""),
    };
    match fs::create_dir("C:/Program Files/P-layer/data/publicmusiclist"){
        Ok(ok)=>ok,
        Err(err)=>eprintln!(""),
    };
    match fs::create_dir("C:/Program Files/P-layer/data/user-avatar"){
        Ok(ok)=>ok,
        Err(err)=>eprintln!(""),
    };
    // let url=format!("{}api/download",SERVER_URL);
    // RequestLib::download(&url,"2146265126","avatar.png","C:/Program Files/P-layer/data/user-avatar/avatar.png").await;
}
async fn update_my_music_list(SERVER_URL:String,useraccount:String,cargo_worker:tokio::sync::mpsc::UnboundedSender<CargoMessage>){
    let jsonstr1=match RequestLib::get(&format!("{}api/getmusiclist/{}/all",SERVER_URL,useraccount)).await{
        Ok(ok)=>ok,
        Err(err)=>todo!(),
    };
    let musicList: Vec<MusicList> = serde_json::from_str(jsonstr1.as_str()).unwrap();
    println!("{:?}",musicList);
    
    //let mut musiclist1=musicList.clone();
    let mut musiclist2: Vec<(slint::SharedString, slint::SharedString, bool, slint::SharedString)> = Vec::new();
    for list in musicList{
        println!("{:?}",list);
        musiclist2.push((list.date.clone().into(),list.name.clone().into(),list.public.clone(),list.user.clone().into()));
        let path=format!("C:/Program Files/P-layer/data/musiclist/{},{},{},{}",<std::string::String as Into<String>>::into(list.user.clone().into()),<std::string::String as Into<String>>::into(list.name.clone().into()),list.date,list.public);
        FileLib::writefile(path.as_str(), b"").await;
    };
    println!("{:?}",musiclist2);
    
    cargo_worker.send(CargoMessage::SetValue("set_musiclist".to_string(),"1".to_string(),Some(musiclist2))).unwrap();


}

async fn update_pub_music_list(SERVER_URL:String,cargo_worker:tokio::sync::mpsc::UnboundedSender<CargoMessage>){

    let publicjsonstr=RequestLib::get(format!("{}api/getmusiclistall/true",SERVER_URL).as_str()).await.unwrap();
    //println!("{}",jsonstr);
    let publicmusiclist: Vec<MusicList> = serde_json::from_str(publicjsonstr.as_str()).unwrap();
    println!("{:?}",publicmusiclist);
    let mut publicmusicList: Vec<(slint::SharedString, slint::SharedString, bool, slint::SharedString)> = Vec::new();
    for list in publicmusiclist{
        println!("{:?}",list);
        publicmusicList.push((list.date.clone().into(),list.name.clone().into(),list.public.clone(),list.user.clone().into()));
        let path=format!("C:/Program Files/P-layer/data/publicmusicList/{},{},{}",<std::string::String as Into<String>>::into(list.user.clone().into()),<std::string::String as Into<String>>::into(list.name.clone().into()),list.date);
        FileLib::writefile(path.as_str(), b"").await;
    };

    cargo_worker.send(CargoMessage::SetValue("set_publicmusic".to_string(),"1".to_string(),Some(publicmusicList))).unwrap();
    
}