
use std::sync::Arc;
use slint::ComponentHandle;
mod call_backs{
    pub mod main_call_backs;
    pub mod music_call_backs;
    pub mod messages_call_backs;
}
mod per_ready;

use crate::{AppWindow, musicplayer::MusicPlayer, slinttrait::SlintSender};
pub async fn apprun(ui: &AppWindow, musicplayer:Arc<MusicPlayer>,slint_sender:Arc<SlintSender>) {
    per_ready::main(ui,slint_sender.clone()).await;
    call_backs::main_call_backs::main(ui,slint_sender.clone());
    call_backs::music_call_backs::main(ui,musicplayer);
    call_backs::messages_call_backs::main(ui,slint_sender.clone());

    ui.run().unwrap();
}


