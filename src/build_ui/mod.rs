use crate::save_window_size::save_window_size;
use std::collections::HashMap;
use std::process::Command;
use std::thread;
use adw::{gio, glib};
use adw::glib::{clone, MainContext, Priority};
use gtk::Orientation;
use gtk::prelude::{BoxExt, ButtonExt, FrameExt, GtkWindowExt, WidgetExt};
use crate::{DriverPackage, PROJECT_VERSION};
use adw::prelude::{*};

pub fn build_ui(app: &adw::Application) {
    gtk::glib::set_prgname(Some("Pika Drivers"));
    glib::set_application_name("Pika Drivers");
    let glib_settings = gio::Settings::new("com.pika.drivers");

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
        .icon_name("pika-drivers")
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
        .icon_name("pika-drivers")
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



    let credits_window_box =  gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let credits_icon = gtk::Image::builder()
        .icon_name("pika-drivers")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .pixel_size(128)
        .build();

    let credits_label = gtk::Label::builder()
        .label("Pika Drivers\nMade by: Cosmo")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let credits_frame = gtk::Frame::builder()
        .margin_top(8)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let window_title_bar = gtk::HeaderBar::builder()
        .show_title_buttons(true)
        .build();

    let credits_button = gtk::Button::builder()
        .icon_name("dialog-information-symbolic")
        .build();

    credits_frame.set_label_align(0.5);

    credits_frame.set_label(Some(PROJECT_VERSION));

    let credits_window = adw::Window::builder()
        .content(&credits_window_box)
        .transient_for(&window)
        .resizable(false)
        .hide_on_close(true)
        .icon_name("pika-drivers")
        .startup_id("pika-drivers")
        .build();

    credits_window_box.append(&gtk::HeaderBar::builder().show_title_buttons(true).build());
    credits_window_box.append(&credits_icon);
    credits_window_box.append(&credits_label);
    credits_window_box.append(&credits_frame);

    content_box.append(&window_title_bar);
    content_box.append(&loading_box);

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
    drive_hws_main_context.spawn_local(clone!(@weak content_box, @weak loading_box => async move {
        while let Ok(drive_hws_state) = drive_hws_receiver.recv().await {
            get_drivers(&content_box, &loading_box, drive_hws_state);
        }
    }));



}

fn get_drivers(main_window: &gtk::Box, loading_box: &gtk::Box, ubuntu_drivers_list_utf8: String) {
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

            //
            drivers_list_row.append(&driver_expander_row);
        }
    }
    main_window.remove(loading_box);
    main_window.append(&window_box);
}