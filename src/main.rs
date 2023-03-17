

slint::include_modules!();

fn main() {
    let main_window = MainWindow::new();
    main_window.set_custom_min_height(400.0);
    let window = main_window.window();
    // window.on_close_requested(|| {
    //     dbg!("test");
    // });
    main_window.run();
}
