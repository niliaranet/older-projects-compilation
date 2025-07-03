use gtk::prelude::*;

pub fn get() -> gtk::Widget {
    let video = gtk::Video::new();
    video.set_vexpand(true);

    if let Some(file) = Some(gtk::gio::File::for_uri(
        "https://giraffe.niliara.net/api/video/source/1",
    )) {
        video.set_file(Some(&file));
    }

    return video.upcast();
}
