use std::{thread, time};

use std::{
    error::Error,
};

use duct::cmd;
use std::io::prelude::*;
use std::io::BufReader;
use crate::save_window_size::save_window_size;
use std::collections::HashMap;
use std::process::Command;
use adw::{gio, glib, Window};
use adw::glib::{clone, MainContext, Priority};
use gtk::Orientation;
use gtk::prelude::{BoxExt, ButtonExt, FrameExt, GtkWindowExt, WidgetExt};
use crate::{DriverPackage, APP_ICON, VERSION, APP_DEV};
use adw::prelude::{*};

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

pub fn build_ui(app: &adw::Application) {
    gtk::glib::set_prgname(Some("Pika Drivers"));
    glib::set_application_name("Pika Drivers");
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
        .title("PikaOS Driver Manager")
        .application(app)
        .content(&content_box)
        .icon_name(APP_ICON)
        .default_width(glib_settings.int("window-width"))
        .default_height(glib_settings.int("window-height"))
        .width_request(900)
        .height_request(500)
        .startup_id("pika-drivers")
        .hide_on_close(true)
        .build();

    if glib_settings.boolean("is-maximized") == true {
        window.maximize()
    }

    let window_title_bar = gtk::HeaderBar::builder()
        .show_title_buttons(true)
        .build();

    let credits_button = gtk::Button::builder()
        .icon_name("dialog-information-symbolic")
        .build();

    let credits_window = adw::AboutWindow::builder()
        .icon_name(APP_ICON)
        .transient_for(&window)
        .version(VERSION)
        .hide_on_close(true)
        .developer_name(APP_DEV)
        .build();

    window_title_bar.pack_end(&credits_button.clone());

    window.connect_hide(clone!(@weak window => move |_| save_window_size(&window, &glib_settings)));
    window.connect_hide(clone!(@weak window => move |_| window.destroy()));

    credits_button.connect_clicked(clone!(@weak credits_button => move |_| credits_window.show()));

    let (drive_hws_sender, drive_hws_receiver) = async_channel::unbounded();
    let drive_hws_sender = drive_hws_sender.clone();
    // The long running operation runs now in a separate thread
    gio::spawn_blocking(move || {
        println!("Checking HW paramter script for available drivers:\n");
        let ubuntu_drivers_list_cli = Command::new("/usr/lib/pika/drivers/generate_driver_definitions.sh")
            .output()
            .expect("failed to execute process");

        let ubuntu_drivers_list_utf8 = String::from_utf8(ubuntu_drivers_list_cli.stdout).unwrap();
        drive_hws_sender
            .send_blocking(ubuntu_drivers_list_utf8)
            .expect("The channel needs to be open.");
    });

    window.show();

    let drive_hws_main_context = MainContext::default();
    // The main loop executes the asynchronous block
    drive_hws_main_context.spawn_local(clone!(@weak content_box, @weak loading_box, @weak window => async move {
        while let Ok(drive_hws_state) = drive_hws_receiver.recv().await {
            get_drivers(&content_box, &loading_box, drive_hws_state, &window);
        }
    }));



}

fn get_drivers(main_window: &gtk::Box, loading_box: &gtk::Box, ubuntu_drivers_list_utf8: String, window: &adw::ApplicationWindow) {
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

    for ubuntu_driver in ubuntu_drivers_list_utf8.lines() {
        let ubuntu_driver_string = ubuntu_driver.to_string();
        let command_version_label = Command::new("/usr/lib/pika/drivers/generate_package_info.sh")
            .args(["version", ubuntu_driver])
            .output()
            .unwrap();
        let command_description_label = Command::new("/usr/lib/pika/drivers/generate_package_info.sh")
            .args(["description", ubuntu_driver])
            .output()
            .unwrap();
        let command_device_label = Command::new("/usr/lib/pika/drivers/generate_package_info.sh")
            .args(["device", ubuntu_driver])
            .output()
            .unwrap();
        let command_icon_label = Command::new("/usr/lib/pika/drivers/generate_package_info.sh")
            .args(["icon", ubuntu_driver])
            .output()
            .unwrap();
        let command_safe_label = Command::new("/usr/lib/pika/drivers/generate_package_info.sh")
            .args(["safe", ubuntu_driver])
            .output()
            .unwrap();
        let ubuntu_driver_package = DriverPackage {
            driver: ubuntu_driver_string,
            version: String::from_utf8(command_version_label.stdout).unwrap().trim().to_string(),
            device: String::from_utf8(command_device_label.stdout).unwrap().trim().to_string(),
            description: String::from_utf8(command_description_label.stdout).unwrap().trim().to_string(),
            icon: String::from_utf8(command_icon_label.stdout).unwrap().trim().to_string(),
            experimental: String::from_utf8(command_safe_label.stdout).unwrap().trim().parse().unwrap(),
        };
        driver_array.push(ubuntu_driver_package);
        driver_array.sort_by(|a, b| b.cmp(a))
    }

    driver_array.into_iter().for_each(|driver_package| {
        let group = device_groups.entry(driver_package.clone().device).or_insert(vec![]);
        group.push(driver_package);
    });
    for (device, group) in device_groups {
        let device_label = gtk::Label::builder()
            .label("Device: ".to_owned() + &device)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .build();
        device_label.add_css_class("deviceLabel");

        main_box.append(&device_label);

        let drivers_list_row = gtk::ListBox::builder()
            .margin_top(20)
            .margin_bottom(20)
            .margin_start(20)
            .margin_end(20)
            .vexpand(true)
            .hexpand(true)
            .build();
        drivers_list_row.add_css_class("boxed-list");

        main_box.append(&drivers_list_row);

        for driver in group.iter() {
            let (log_loop_sender, log_loop_receiver) = async_channel::unbounded();
            let log_loop_sender: async_channel::Sender<String> = log_loop_sender.clone();

            let (log_status_loop_sender, log_status_loop_receiver) = async_channel::unbounded();
            let log_status_loop_sender: async_channel::Sender<bool> = log_status_loop_sender.clone();
            let driver_expander_row = adw::ExpanderRow::new();
            let driver_icon = gtk::Image::builder()
                .icon_name(driver.clone().icon)
                .pixel_size(32)
                .build();
            let driver_description_label = gtk::Label::builder()
                .label(driver.clone().description)
                .build();
            let driver_content_row = adw::ActionRow::builder()
                .build();
            let driver_install_button = gtk::Button::builder()
                .margin_start(5)
                .margin_top(5)
                .margin_bottom(5)
                .valign(gtk::Align::Center)
                .label("Install")
                .tooltip_text("Install the driver package.")
                .sensitive(false)
                .build();
            driver_install_button.add_css_class("suggested-action");
            let driver_remove_button = gtk::Button::builder()
                .margin_end(5)
                .margin_top(5)
                .margin_bottom(5)
                .valign(gtk::Align::Center)
                .label("Uninstall")
                .tooltip_text("Uninstall the driver package.")
                .sensitive(false)
                .build();
            let driver_action_box = gtk::Box::builder()
                .homogeneous(true)
                .build();
            driver_remove_button.add_css_class("destructive-action");
            driver_expander_row.add_prefix(&driver_icon);
            if driver.clone().experimental == true {
                driver_expander_row.set_title(&(driver.clone().driver + " (WARNING: THIS DRIVER IS EXPERMINTAL USE AT YOUR OWN RISK!)"));
                driver_expander_row.add_css_class("midLabelWARN");
            } else {
                driver_expander_row.set_title(&driver.clone().driver);
            }
            driver_expander_row.set_subtitle(&driver.clone().version);
            //
            driver_content_row.add_prefix(&driver_description_label);
            driver_action_box.append(&driver_remove_button);
            driver_action_box.append(&driver_install_button);
            driver_content_row.add_suffix(&driver_action_box);
            driver_expander_row.add_row(&driver_content_row);
            //
            let command_installed_status = Command::new("dpkg")
                .args(["-s", &driver.clone().driver])
                .output()
                .unwrap();
            if command_installed_status.status.success() {
                driver_install_button.set_sensitive(false);
                if !driver.clone().driver.contains("mesa") {driver_remove_button.set_sensitive(true);}
            } else {
                driver_remove_button.set_sensitive(false);
                driver_install_button.set_sensitive(true);
            }
            //
            let driver_install_log_terminal_buffer = gtk::TextBuffer::builder().build();

            let driver_install_log_terminal = gtk::TextView::builder()
                .vexpand(true)
                .hexpand(true)
                .editable(false)
                .buffer(&driver_install_log_terminal_buffer)
                .build();

            let driver_install_log_terminal_scroll = gtk::ScrolledWindow::builder()
                .width_request(400)
                .height_request(200)
                .vexpand(true)
                .hexpand(true)
                .child(&driver_install_log_terminal)
                .build();

            let driver_install_dialog = adw::MessageDialog::builder()
                .transient_for(window)
                .hide_on_close(true)
                .extra_child(&driver_install_log_terminal_scroll)
                .width_request(400)
                .height_request(200)
                .heading("driver_install_dialog_heading")
                .build();
            driver_install_dialog.add_response("driver_install_dialog_ok", "system_update_dialog_ok_label");

            let log_loop_context = MainContext::default();
            // The main loop executes the asynchronous block
            log_loop_context.spawn_local(clone!(@weak driver_install_log_terminal_buffer, @weak driver_install_dialog => async move {
            while let Ok(state) = log_loop_receiver.recv().await {
                driver_install_log_terminal_buffer.insert(&mut driver_install_log_terminal_buffer.end_iter(), &("\n".to_string() + &state))
            }
            }));

                    let log_status_loop_context = MainContext::default();
                    // The main loop executes the asynchronous block
                    log_status_loop_context.spawn_local(clone!(@weak driver_install_dialog => async move {
                    while let Ok(state) = log_status_loop_receiver.recv().await {
                        if state == true {
                            driver_install_dialog.set_response_enabled("driver_install_dialog_ok", true);
                            driver_install_dialog.set_body("driver_install_dialog_success_true");
                        } else {
                            driver_install_dialog.set_response_enabled("driver_install_dialog_ok", true);
                            driver_install_dialog.set_body("driver_install_dialog_success_false");
                        }
                    }
            }));

            driver_install_log_terminal_buffer.connect_changed(clone!(@weak driver_install_log_terminal, @weak driver_install_log_terminal_buffer,@weak driver_install_log_terminal_scroll => move |_|{
               if driver_install_log_terminal_scroll.vadjustment().upper() - driver_install_log_terminal_scroll.vadjustment().value() > 100.0 {
                    driver_install_log_terminal_scroll.vadjustment().set_value(driver_install_log_terminal_scroll.vadjustment().upper())
                }
            }));
            //
            drivers_list_row.append(&driver_expander_row);
        }
    }
    main_window.remove(loading_box);
    main_window.append(&window_box);
}