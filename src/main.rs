use orbtk::prelude::*;
use orbtk::theme::DEFAULT_THEME_CSS;

mod prelude;
mod header;
mod body;

use header::render_header;
use body::*;
use prelude::*;

fn get_theme() -> ThemeValue {
    ThemeValue::create_from_css(DEFAULT_THEME_CSS)
        .extension_css(include_str!("../res/grid.css"))
        .build()
}

widget!(MainView<MainViewState> {
    text: String16
});

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let body_data = Body::init(ctx);
        let header_elements = render_header(id, ctx, body_data.get_tabs());
        self.name("MainView").child(
            Grid::create()
                .rows(Rows::create().row(50.0).row("*").build())
                .child(
                    Grid::create()
                        .selector("bluebayoux")
                        .attach(Grid::row(0))
                        .child(header_elements)
                        .build(ctx)
                )
                .child(body_data.get_entity())
                .build(ctx))
    }
}

fn main() {
    Application::new()
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