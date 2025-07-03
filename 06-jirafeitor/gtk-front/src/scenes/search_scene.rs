use gtk::glib;
use gtk::prelude::*;

use super::video_scene;
use crate::models::Video;
use reqwest;
use serde_json;

use crate::AppData;

pub fn get(app_data: &AppData) -> gtk::Widget {
    let box_ = gtk::Box::new(gtk::Orientation::Vertical, 5);
    box_.append(&gtk::Label::new(Some("Search")));

    let box_clone = box_.clone();

    glib::spawn_future_local(async move {
        if let Ok(response) = fetch_data().await {
            for video in response {
                println!("{}, {}", video.name, video.id);
                let btn = gtk::Button::with_label(&video.name);
                let refe = &app_data.current_scene;
                btn.connect_clicked(move |_| refe = video_scene.get());
                box_clone.append(&btn);
            }
        }
    });

    box_.upcast()
}

async fn fetch_data() -> Result<Vec<Video>, reqwest::Error> {
    println!("doin");
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let data = reqwest::get("https://giraffe.niliara.net/api/videos")
            .await?
            .text()
            .await?;

        println!("{}", data);
        let result: Vec<Video> = serde_json::from_str(&data).unwrap_or_else(|_| Vec::new());

        println!("{:?}", result);
        return Ok(result);
    })
}
