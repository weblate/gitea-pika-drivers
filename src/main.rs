use std::process::Command;
use gtk::prelude::*;
use gtk::*;
use glib::*;

fn main() {
    let app = Application::builder()
        .application_id("com.pika.drivers")
        .build();
        
    app.connect_activate(build_ui);
    app.run();
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
        let driver_string2 = driver.to_string();
    
        let driver_box = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .build();
    
        
        let driver_label = gtk::Label::builder()
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .build();
        
        if driver_name != "emScuM8rsa6kuhMePtR5bT8s4z9s" {
                driver_label.set_label(driver)
        } else {
                driver_label.set_label("No Driver are required for this system you are good to go! 😎")
        }
        
        

        
        let driver_button = gtk::Button::builder()
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();

        let driver_separator = gtk::Separator::builder()
            .hexpand(true)
            .opacity(0.0)
            .build();
        
        driver_button_refresh(&driver_string2, &driver_button);
        
        driver_box.append(&driver_label);
        driver_box.append(&driver_separator);
        if driver_name != "emScuM8rsa6kuhMePtR5bT8s4z9s" {
            driver_box.append(&driver_button);
        }
        drivers_list_row.append(&driver_box);
        
        driver_button.connect_clicked(clone!(@weak driver_button => move |_| modify_package(&driver_string, &driver_button)));
    
    }

        
    let main_scroll =  gtk::ScrolledWindow::builder()
        .child(&main_box)
        .build();
    let window = gtk::ApplicationWindow::builder()
        .title("PikaOS Driver Manager")
        .application(app)
        .child(&main_scroll)
        .icon_name("mintinstall")
        .default_width(700)
        .default_height(500)
        .width_request(500)
        .height_request(500)
        .startup_id("pika-drivers")
        .build();
        
        
    window.show()
}

    
fn modify_package(package: &str, driver_button: &Button) {
    println!("Start installing driver {}: ", package);
    let wrapper_command = Command::new("x-terminal-emulator")
            .args(["-e", "bash", "-c", "/usr/lib/pika/drivers/modify-driver.sh", package])
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
            .args(["-c", "/usr/lib/pika/drivers/dialog-error.sh", package])
            .output()
            .unwrap();
    }
}

fn driver_button_refresh(driver: &str, driver_button: &Button) {
    let  driver_command = Command::new("dpkg")
        .args(["-s", driver])
        .output()
        .unwrap();
    if driver_command.status.success() {
            println!("Checking Driver Presence of {}: Success!", driver);
            if driver.contains("nvidia") {
                driver_button.set_tooltip_text(Some("This uninstall and revert to nouveau"));
            } else {
                driver_button.set_tooltip_text(Some("This uninstall the driver"));
            }
                driver_button.set_icon_name("user-trash-symbolic");
    } else {
            println!("Checking Driver Presence of {}: Failure! Driver isn't installed", driver);
            if driver.contains("nvidia") {
                driver_button.set_tooltip_text(Some("This will remove nouveau and install"));
            } else {
                driver_button.set_tooltip_text(Some("Install"));
            }
            driver_button.set_icon_name("go-down-symbolic");
    }
}
