use gtk::Align;
use gtk::prelude::*;

pub fn get() -> gtk::Widget {
    let top_bar = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    top_bar.set_valign(Align::Start);
    top_bar.set_margin_top(5);

    let jirafeitor_logo = gtk::Image::from_file("./assets/giraffe.png");
    jirafeitor_logo.set_margin_end(10);
    jirafeitor_logo.set_margin_start(20);
    jirafeitor_logo.set_pixel_size(50);
    top_bar.append(&jirafeitor_logo);

    let title = gtk::Label::new(Some("Jirafeitor"));
    title.set_halign(Align::Start);
    title.set_hexpand(true);
    top_bar.append(&title);

    let search_entry = gtk::SearchEntry::new();
    search_entry.set_placeholder_text(Some("Search..."));
    search_entry.set_max_width_chars(40);
    search_entry.set_margin_start(20);
    search_entry.set_margin_end(20);
    search_entry.set_margin_top(10);
    search_entry.set_margin_bottom(10);
    top_bar.append(&search_entry);

    return top_bar.upcast();
}
