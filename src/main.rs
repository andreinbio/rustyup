use orbtk::prelude::*;
use orbtk::theme::DEFAULT_THEME_CSS;

mod main_state;
mod main_view;

mod header;
mod body;
mod update;
mod appsettings;

use main_state::*;
use main_view::*;

use header::render_header;
use body::*;

fn get_theme() -> ThemeValue {
    ThemeValue::create_from_css(DEFAULT_THEME_CSS)
        .extension_css(include_str!("../res/grid.css"))
        .build()
}

fn main() {
    Application::from_name("rustyup")
        .window(|ctx| {
            Window::create()
                .title("Test")
                .position((-8.0, 0.0))
                .size(1920.0, 1080.0)
                .theme(get_theme())
                .resizeable(true)
                .child(MainView::create().build(ctx))
                .build(ctx)
        })
        .run();
}