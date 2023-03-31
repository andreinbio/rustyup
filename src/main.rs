
extern crate serde;

mod config;
mod appsettings;
use appsettings::AppSettings;

slint::include_modules!();

fn update_window_settings(main_window: &MainWindow) -> () {
    let settings: AppSettings = appsettings::load();
    
    main_window.set_custom_min_height(600.0);
}

fn save_window_settings(main_window: &MainWindow) -> () {
    let new_window: &slint::Window = main_window.window();
    let position: slint::PhysicalPosition = new_window.position();
    let size: slint::PhysicalSize = new_window.size();
    dbg!(&position.x);
    dbg!(&position.y);
    dbg!(&size.width);
    dbg!(&size.height);
    // appsettings::save();
}

fn main() {
    let main_window: MainWindow = MainWindow::new();
    update_window_settings(&main_window);
    let window: &slint::Window = main_window.window();
    let weak_window: slint::Weak<MainWindow> = main_window.as_weak();

     let on_window_close_callback = move || {
        let new_option_window: Option<MainWindow> = weak_window.upgrade();

        if new_option_window.is_some() {
            let new_main_window: MainWindow = new_option_window.unwrap();
            save_window_settings(&new_main_window);
        }

        return slint::CloseRequestResponse::HideWindow;
    };

    window.on_close_requested(on_window_close_callback);

    main_window.run();
}
