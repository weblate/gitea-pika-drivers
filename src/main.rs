use std::process::Command;
use gtk::prelude::*;
use gtk::*;
use glib::*;
use gdk::Display;


const PROJECT_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let application = Application::new(Some("com.pika.drivers"), Default::default());
    application.connect_startup(|app| {
        // The CSS "magic" happens here.
        let provider = CssProvider::new();
        provider.load_from_data(include_str!("/usr/lib/pika/drivers/style.css"));
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


fn build_ui(app: &Application) {


    let main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
        
    let drivers_list_row = gtk::ListBox::builder()
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .vexpand(true)
        .hexpand(true)
        .show_separators(true)
        .build();
        
    main_box.append(&drivers_list_row);

    println!("Checking HW paramter script for available drivers:\n");
    let ubuntu_drivers_list_cli = Command::new("/usr/lib/pika/drivers/generate_driver_definitions.sh")
                         .output()
                         .expect("failed to execute process");
    
    let ubuntu_drivers_list_utf8 = String::from_utf8(ubuntu_drivers_list_cli.stdout).unwrap();

    for driver in ubuntu_drivers_list_utf8.lines() {
    
    
        let driver_name = driver;
    
        let driver_string = driver.to_string();
    
        let driver_box = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .build();
    
        
        
        let driver_start_part_box = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .width_request(350)
            .build();
        
        let driver_icon_label_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .build();
        
        let driver_label = gtk::Label::builder()
                .margin_top(12)
                .build();
        driver_label.add_css_class("startLabel");

                
        let driver_icon = gtk::Image::builder()
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .pixel_size(64)
                .halign(Align::Start)
                .build();
        
        if driver_name != "emScuM8rsa6kuhMePtR5bT8s4z9s" {
                driver_label.set_label(driver);
                if driver_name == "pika-rocm-meta" {
                    driver_icon.set_from_icon_name(Some("amd"));
                } else if driver_name == "vulkan-amdgpu-pro" {
                    driver_icon.set_from_icon_name(Some("amd"));
                } else if driver_name == "amf-amdgpu-pro" {
                    driver_icon.set_from_icon_name(Some("amd"));
                } else if driver_name == "amdvlk" {
                    driver_icon.set_from_icon_name(Some("amd"));
                } else if driver_name == "opencl-legacy-amdgpu-pro-icd" {
                    driver_icon.set_from_icon_name(Some("amd"));
                } else if driver_name == "amdgpu-pro-oglp" {
                    driver_icon.set_from_icon_name(Some("amd"));
                } else if driver_name == "xone-dkms" {
                    driver_icon.set_from_icon_name(Some("input-gaming"));
                } else if driver_name.contains("nvidia") {
                    driver_icon.set_from_icon_name(Some("nvidia"));
                } else {
                    driver_icon.set_from_icon_name(Some("pika-drivers"));
                }
        } else {
                driver_label.set_label("No Drivers are required for this system you are good to go! 😎");
                driver_icon.hide()
        }
        
        let  command_version_label = Command::new("/usr/lib/pika/drivers/generate_package_info.sh")
            .args(["version", driver.clone()])
            .output()
            .unwrap();
            
        let  command_description_label = Command::new("/usr/lib/pika/drivers/generate_package_info.sh")
            .args(["description", driver.clone()])
            .output()
            .unwrap();
            
        let  command_device_label = Command::new("/usr/lib/pika/drivers/generate_package_info.sh")
            .args(["device", driver.clone()])
            .output()
            .unwrap();

        
        let driver_button_icon = gtk::Image::builder()
            .pixel_size(24)
            .build();
            
        let driver_end_part_box = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .build();

        let driver_device_icon = gtk::Image::builder()
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .icon_name("edit-select-all-symbolic")
            .build();

        let driver_version_icon = gtk::Image::builder()
            .margin_bottom(12)
            .icon_name("dialog-question-symbolic")
            .halign(Align::Start)
            .build();

        let driver_middle_part_box = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .hexpand(true)
            .build();
            
        let driver_middle_part_description_label = gtk::Label::builder()
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .hexpand(true)
            .justify(Justification::Left)
            .wrap(true)
            .build();
        driver_middle_part_description_label.add_css_class("midLabel");
        driver_version_icon.set_tooltip_text(Some(&String::from_utf8(command_version_label.stdout).unwrap()));
        driver_device_icon.set_tooltip_text(Some(&String::from_utf8(command_device_label.stdout).unwrap()));
        driver_middle_part_description_label.set_text(&String::from_utf8(command_description_label.stdout).unwrap());
        
        driver_button_refresh(&driver_string.clone(), &driver_button_icon);
        
        driver_middle_part_box.append(&driver_middle_part_description_label);
        driver_middle_part_box.append(&driver_device_icon);
        
        
        
        let driver_button = gtk::Button::builder()
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .has_frame(false)
            .child(&driver_button_icon)
            .build();
        
        let driver_start_sep = gtk::Separator::builder()
            .build();
            
        let driver_end_sep = gtk::Separator::builder()
            .build();
            
        driver_icon_label_box.append(&driver_label);
        driver_icon_label_box.append(&driver_version_icon);
        driver_start_part_box.append(&driver_icon);
        driver_start_part_box.append(&driver_icon_label_box);
        driver_box.append(&driver_start_part_box);
        driver_box.append(&driver_start_sep);
        driver_box.append(&driver_middle_part_box);
        driver_box.append(&driver_end_sep);
        driver_box.append(&driver_end_part_box);
        if driver_name != "emScuM8rsa6kuhMePtR5bT8s4z9s" {
            driver_end_part_box.append(&driver_button);
        }
        drivers_list_row.append(&driver_box);
        
        driver_button.connect_clicked(clone!(@weak driver_button => move |_| modify_package(&driver_string, &driver_button_icon)));
    
    }

        
    let window_box  =  gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    
    let main_scroll =  gtk::ScrolledWindow::builder()
        .child(&main_box)
        .build();
        
    let window_title_bar = gtk::HeaderBar::builder()
        .show_title_buttons(true)
        .build();
        
    let credits_button = gtk::Button::builder()
        .icon_name("dialog-information-symbolic")
        .build();
    
    window_box.append(&main_scroll);
    
    

    let window = gtk::ApplicationWindow::builder()
        .title("PikaOS Driver Manager")
        .application(app)
        .child(&window_box)
        .icon_name("mintinstall")
        .default_width(1200)
        .default_height(600)
        .width_request(500)
        .height_request(500)
        .startup_id("pika-drivers")
        .build();
        
        
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
        .label("Pika Drivers")
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
        .label_align(Align::Center)
        .build();
        
    credits_frame.set_label(Some(PROJECT_VERSION));
        
    let credits_window = gtk::Window::builder()
        .child(&credits_window_box)
        .transient_for(&window)
        .build();
        
    credits_window_box.append(&credits_icon);
    credits_window_box.append(&credits_label);
    credits_window_box.append(&credits_frame);
        
    window.set_titlebar(Some(&window_title_bar));
    
    window_title_bar.pack_end(&credits_button);
    
    credits_button.connect_clicked(clone!(@weak credits_button => move |_| credits_window.show()));
        
    window.show()
}

    
fn modify_package(package: &str, driver_button: &Image) {
    let str_pkg = package.to_string();
    println!("Start installing driver {}: ", package);
    let wrapper_command = Command::new("x-terminal-emulator")
            .arg("-e")
            .arg("bash")
            .arg("-c")
            .arg("/usr/lib/pika/drivers/modify-driver.sh \"$1\"")
            .arg("bash") // $0
            .arg(&str_pkg) // $1
            .output()
            .unwrap();
    if wrapper_command.status.success() {
        println!("Installation Command has ended.\n");
        println!("Installation was successful!\n");
        println!("Refreshing GUI Labels.\n");
        driver_button_refresh(package, driver_button);
    } else {
        println!("Installation Command has ended.\n");
        println!("Installation was failed :(\n");
        println!("Refreshing GUI Labels.\n");
        driver_button_refresh(package, driver_button);
        println!("Sending error message.\n");
        let _error_command = Command::new("bash")
            .arg("-c")
            .arg("/usr/lib/pika/drivers/dialog-error.sh \"$1\"")
            .arg("bash") // $0
            .arg(&str_pkg) // $1
            .output()
            .unwrap();
    }
}

fn driver_button_refresh(driver: &str, driver_button: &Image) {
    let  driver_command = Command::new("dpkg")
        .args(["-s", driver])
        .output()
        .unwrap();
    if driver_command.status.success() {
            println!("Checking Driver Presence of {}: Success!", driver);
            if driver.contains("nvidia") {
                driver_button.set_tooltip_text(Some("Uninstall and revert to nouveau."));
            } else {
                driver_button.set_tooltip_text(Some("Uninstall."));
            }
                driver_button.set_icon_name(Some("user-trash-symbolic"));
    } else {
            println!("Checking Driver Presence of {}: Failure! Driver isn't installed", driver);
            if driver.contains("nvidia") {
                driver_button.set_tooltip_text(Some("Install and override nouveau."));
            } else {
                driver_button.set_tooltip_text(Some("Install."));
            }
            driver_button.set_icon_name(Some("go-down-symbolic"))
    }
}
