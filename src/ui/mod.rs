
use std::sync::Arc;



use slint::ComponentHandle;
mod call_backs{
    pub mod main_call_backs;
    pub mod music_call_backs;
    pub mod messages_call_backs;
}
mod per_ready;


// #[derive(Debug)]
// struct User {
//     user_token:String,
//     user_id: String,
//     user_account: String,
//     user_password: String,
//     user_name: String,
//     user_gender: String,
//     user_age: String,
//     user_intro: String,
//     user_sign_date: String,
//     user_music_number: i32,
// }



#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct MusicList {
    user: String,
    name: String,
    date: String,
    public: bool,
}

use crate::{AppWindow, musicplayer::MusicPlayer, slinttrait::SlintSender};
pub async fn apprun(ui: &AppWindow, musicplayer:Arc<MusicPlayer>,slint_sender:Arc<SlintSender>) {
    //let musicplayerarc1=musicplayer.clone();
    per_ready::main(ui,slint_sender.clone()).await;
    call_backs::main_call_backs::main(ui,slint_sender.clone());
    call_backs::music_call_backs::main(ui,musicplayer);
    call_backs::messages_call_backs::main(ui,slint_sender.clone());

    ui.run().unwrap();
}


