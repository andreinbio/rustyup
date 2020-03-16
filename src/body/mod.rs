use orbtk::prelude::*;

mod content;
mod settings;
mod utils;

use settings::*;
use content::*;
use utils::*;

pub fn render_body(id: Entity, ctx: &mut BuildContext) -> Entity {
    Grid::create()
        .attach(Grid::row(1))
        .clip(true)
        .child(
            ItemsWidget::create()
                .id("body_content")
                .items_builder(move |bc, index| {
                    get_body_entity(id, bc)
                })
                .count(1)
                .build(ctx)
        )
        .build(ctx)
}

fn get_body_entity(id: Entity, ctx: &mut BuildContext) -> Entity {
    let active_tab: usize = ctx.get_widget(id).get::<usize>("active_tab").clone();
    let mut grid = Grid::create();
    let body_content: Grid = if active_tab == 0 {
        create_settings_body(id, ctx)
    } else {
        create_content_body(id, ctx)
    };

    grid = grid.child(
        body_content
            .build(ctx)
    );

    grid.build(ctx)
}
