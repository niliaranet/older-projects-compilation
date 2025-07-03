use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

mod components;
mod models;
mod scenes;

pub struct AppData {
    main: gtk::Box,
    current_scene: gtk::Widget,
}

impl AppData {
    fn default() -> AppData {
        return AppData {
            main: gtk::Box::new(gtk::Orientation::Vertical, 0),
            current_scene: scenes::video_scene::get(),
        };
    }

    fn setup(&mut self) {
        self.main.append(&components::topbar::get());
        self.current_scene = scenes::search_scene::get(&self.non_mutable());
    }

    fn change_scene(&mut self) {
        self.current_scene = scenes::video_scene::get();
    }

    fn non_mutable(&self) -> &AppData {
        return self;
    }
}

pub fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Jirafeitor")
        .default_width(800)
        .default_height(600)
        .build();

    let app_data = &mut AppData::default();
    app_data.setup();
    window.set_child(Some(&app_data.main));

    window.show();
}

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("net.niliara.jirafeitor")
        .build();

    app.connect_activate(build_ui);
    app.run()
}
