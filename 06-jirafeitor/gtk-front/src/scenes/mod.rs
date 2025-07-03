use gtk::prelude::*;

pub mod search_scene;
pub mod video_scene;

pub fn get() -> gtk::Widget {
    let scene = gtk::Stack::new();

    let video = video_scene::get();
    scene.add_child(&video);

    let search = search_scene::get();
    scene.add_child(&search);

    scene.set_visible_child(if true { &video } else { &search });
    return scene.upcast();
}
