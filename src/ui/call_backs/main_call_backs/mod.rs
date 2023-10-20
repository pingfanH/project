use std::sync::Arc;

use slint::{ComponentHandle, LogicalPosition};
use crate::slinttrait::{SlintSender,SlintMessage};
use crate::{AppWindow, ultis};
use crate::env::{CLICKSOUND1,CLICKSOUND2};
pub fn main(ui:&AppWindow,slintsender:Arc<SlintSender>){
    let handle=ui.as_weak();
    let hanel_close=ui.as_weak();
    //关闭窗口
    ui.on_close_window(move ||{
        ultis::musicplay(crate::env::CLICKSOUND2);
        hanel_close.upgrade().unwrap().hide().unwrap();
    });
    //跳转到url
    ui.on_open_the_url(|url|{
        println!("{}",url); 
        open::that(url.as_str()).ok();
    });
    ui.on_move_window(move |offset_x, offset_y|{
        let main = handle.upgrade().unwrap();
        //获取窗口的物理坐标
        let logical_pos = main.window().position().to_logical(main.window().scale_factor());
        //窗口坐标添加上偏移量，设置为新的位置
        main.window().set_position(LogicalPosition::new(logical_pos.x + offset_x, logical_pos.y + offset_y));
    });
    //播放点击音效1,2
    ui.on_click_audio(move ||{
        std::thread::spawn(|| {
            crate::ultis::musicplay(CLICKSOUND1);
        });
    });

    ui.on_click_audio1(move ||{
        std::thread::spawn(|| {
            crate::ultis::musicplay(CLICKSOUND2);
        });
    });
    ui.on_test(move|value|{
        let cargo_channel = slintsender.channel.clone();
       // cargo_channel.send(SlintMessage::Test("name".to_owned())).unwrap();
    });
}

