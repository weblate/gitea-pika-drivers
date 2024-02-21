mod save_window_size;
mod build_ui;

use std::collections::HashMap;
use std::process::Command;
use std::thread;
use gtk::prelude::*;
use gtk::*;
use gtk::prelude::*;
use glib::*;
use adw::*;
use gdk::Display;
use adw::prelude::*;

#[derive(PartialEq, Debug, Eq, Hash, Clone, Ord, PartialOrd)]
pub struct DriverPackage {
    driver: String,
    version: String,
    device: String,
    description: String,
    icon: String,
}

const PROJECT_VERSION: &str = env!("CARGO_PKG_VERSION");

use build_ui::build_ui;

fn main() {
    let application = adw::Application::new(Some("com.pika.drivers"), Default::default());
    application.connect_startup(|app| {
        // The CSS "magic" happens here.
        let provider = CssProvider::new();
        provider.load_from_data(include_str!("style.css"));
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        gtk::style_context_add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        app.connect_activate(build_ui);
    });
    
    application.run();
}
