use std::path::Path;
use std::fs;
use crate::config::*;
use crate::save_window_size::save_window_size;
use crate::DriverPackage;
use adw::glib::{clone, MainContext};
use adw::prelude::*;
use adw::{gio, glib};
use duct::cmd;
use gtk::prelude::{BoxExt, ButtonExt, FrameExt, GtkWindowExt, WidgetExt};
use gtk::Orientation;
use std::collections::HashMap;
use std::error::Error;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Index;
use std::process::Command;

use users::*;

use serde_json::json;
use serde_json::Value;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct Ask {
    id: f64,
    driver: String,
    icon: String,
    experimental: bool,
    detection: String
}


pub fn build_ui(app: &adw::Application) {
    gtk::glib::set_prgname(Some(APP_NAME));
    glib::set_application_name(APP_NAME);
    let glib_settings = gio::Settings::new("com.github.pikaos-linux.pikadrivers");

    let content_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .vexpand(true)
        .hexpand(true)
        .build();

    let loading_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .vexpand(true)
        .hexpand(true)
        .build();

    let loading_icon = gtk::Image::builder()
        .icon_name(APP_ICON)
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .vexpand(true)
        .hexpand(true)
        .margin_end(20)
        .pixel_size(256)
        .build();

    let loading_spinner = gtk::Spinner::builder()
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .vexpand(true)
        .hexpand(true)
        .margin_end(20)
        .build();

    let loading_label = gtk::Label::builder()
        .label("Scanning for drivers...")
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .vexpand(true)
        .hexpand(true)
        .margin_end(20)
        .build();

    loading_spinner.start();

    loading_box.append(&loading_icon);
    loading_box.append(&loading_spinner);
    loading_box.append(&loading_label);

    let window = adw::ApplicationWindow::builder()
        .title(APP_NAME)
        .application(app)
        .content(&content_box)
        .icon_name(APP_ICON)
        .default_width(glib_settings.int("window-width"))
        .default_height(glib_settings.int("window-height"))
        .width_request(900)
        .height_request(500)
        .startup_id(APP_ID)
        .hide_on_close(true)
        .build();

    if glib_settings.boolean("is-maximized") == true {
        window.maximize()
    }

    let window_title_bar = gtk::HeaderBar::builder().show_title_buttons(true).build();

    let credits_button = gtk::Button::builder()
        .icon_name("dialog-information-symbolic")
        .build();

    let credits_window = adw::AboutWindow::builder()
        .application_icon(APP_ICON)
        .application_name(APP_NAME)
        .transient_for(&window)
        .version(VERSION)
        .hide_on_close(true)
        .developer_name(APP_DEV)
        .issue_url(APP_GITHUB.to_owned() + "/issues")
        .build();

    content_box.append(&window_title_bar);
    content_box.append(&loading_box);

    window_title_bar.pack_end(&credits_button.clone());

    window.connect_hide(clone!(@weak window => move |_| save_window_size(&window, &glib_settings)));
    window.connect_hide(clone!(@weak window => move |_| window.destroy()));

    credits_button.connect_clicked(clone!(@weak credits_button => move |_| credits_window.present()));

    println!("Downloading driver DB...");
    let data =  reqwest::blocking::get("https://raw.githubusercontent.com/PikaOS-Linux/pkg-pika-drivers/main/driver-db.json").unwrap().text().unwrap();

    let (drive_hws_sender, drive_hws_receiver) = async_channel::unbounded();
    let drive_hws_sender = drive_hws_sender.clone();
    // The long running operation runs now in a separate thread
    gio::spawn_blocking(clone!(@strong data => move || {
        let mut found_driver_ids_array: Vec<i64> = Vec::new();
        println!("Parsing Downloaded driver DB...");
        let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");
        if let serde_json::Value::Array(drivers) = &res["drivers"] {
            for driver in drivers {
                if Path::new("/tmp/run-pkdm-detect.sh").exists() {
                    fs::remove_file("/tmp/run-pkdm-detect.sh").expect("Bad permissions on /tmp/pika-installer-gtk4-target-manual.txt");
                }
                fs::write("/tmp/run-pkdm-detect.sh", "#! /bin/bash\nset -e\n".to_owned() + driver["detection"].as_str().to_owned().unwrap()).expect("Unable to write file");
                let _ = cmd!("chmod", "+x", "/tmp/run-pkdm-detect.sh").run();
                let result = cmd!("/tmp/run-pkdm-detect.sh").run();
                if result.is_ok() {
                    found_driver_ids_array.push(driver["id"].as_i64().unwrap())
                }
            }
        }
        drive_hws_sender
             .send_blocking(found_driver_ids_array)
             .expect("channel needs to be open.")
    }));

    window.present();

    let drive_hws_main_context = MainContext::default();
    // The main loop executes the asynchronous block
    drive_hws_main_context.spawn_local(clone!(@weak content_box, @weak loading_box, @strong data => async move {
        while let Ok(drive_hws_state) = drive_hws_receiver.recv().await {
            get_drivers(&content_box, &loading_box, &drive_hws_state, &window, &data);
        }
    }));
}

const DRIVER_MODIFY_PROG: &str = r###"
#! /bin/bash
DRIVER="$0"
/usr/lib/pika/drivers/modify-driver.sh "${DRIVER}"
"###;
fn driver_modify(
    log_loop_sender: async_channel::Sender<String>,
    driver_pkg: &str,
) -> Result<(), std::boxed::Box<dyn Error + Send + Sync>> {
    let (pipe_reader, pipe_writer) = os_pipe::pipe()?;
    let child = cmd!("bash", "-c", DRIVER_MODIFY_PROG, driver_pkg)
        .stderr_to_stdout()
        .stdout_file(pipe_writer)
        .start()?;
    for line in BufReader::new(pipe_reader).lines() {
        log_loop_sender
            .send_blocking(line?)
            .expect("Channel needs to be opened.")
    }
    child.wait()?;

    Ok(())
}

fn get_drivers(
    main_window: &gtk::Box,
    loading_box: &gtk::Box,
    found_driver_ids_array: &Vec<i64>,
    window: &adw::ApplicationWindow,
    json_data: &String
) {
    let main_box = gtk::Box::builder()
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .orientation(Orientation::Vertical)
        .build();

    let main_scroll = gtk::ScrolledWindow::builder()
        .max_content_width(650)
        .min_content_width(300)
        .child(&main_box)
        .build();

    let window_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    window_box.append(&main_scroll);

    let mut driver_array: Vec<DriverPackage> = Vec::new();
    let mut device_groups: HashMap<String, Vec<DriverPackage>> = HashMap::new();

    main_window.remove(loading_box);
    main_window.append(&window_box);
}
