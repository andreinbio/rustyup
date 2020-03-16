use orbtk::prelude::*;

use crate::{MainState, Action};
use crate::body::utils::*;

pub fn create_settings_body(id: Entity, ctx: &mut BuildContext) -> Grid {
    Grid::create()
        .vertical_alignment(Alignment::Start)
        .clip(true)
        .columns(Columns::create().column(200.0).column("*").build())
        .child(
            get_tab_buttons(id, ctx)
        )
        .child(
            ItemsWidget::create()
                .attach(Grid::column(1))
                .id("settings_tabs")
                .items_builder(move |bc, index| {
                    let active_body_tab: usize = bc.get_widget(id).get::<usize>("active_settings_tab").clone();

                    if active_body_tab == 0 {
                        get_settings_content(id, bc)
                    } else {
                        get_information_content(id, bc)
                    }
                })
                .count(1)
                .build(ctx)
        )
}

// buttons to change between body tabs
fn get_tab_buttons(id: Entity, ctx: &mut BuildContext) -> Entity {
    Stack::create()
        .attach(Grid::column(0))
        .vertical_alignment(Alignment::Start)
        .clip(true)
        .child(
            Button::create()
                .text("Information")
                .on_click(move |states, _| {
                    states.get_mut::<MainState>(id).action(Action::ChangeSettingsTab(1));
                    true
                })
                .build(ctx)
        )
        .child(
            Button::create()
                .text("Configuration")
                .on_click(move |states, _| {
                    states.get_mut::<MainState>(id).action(Action::ChangeSettingsTab(0));
                    true
                })
                .build(ctx)
        )
        .build(ctx)
}

// settings content for each tab
fn get_settings_content(id: Entity, ctx: &mut BuildContext) -> Entity {
    Stack::create()
        .attach(Grid::column(1))
        .horizontal_alignment(Alignment::Stretch)
        .child(create_button_row(id, ctx, "hostname"))
        .child(create_button_row(id, ctx, "username"))
        .child(create_button_row(id, ctx, "password"))
        .child(create_button_row(id, ctx, "code-version"))
        // .child(create_cartridges_data(id, ctx, "cartridge"))
        .build(ctx)
}

// data content for body
fn get_information_content(id: Entity, ctx: &mut BuildContext) -> Entity {
    Button::create()
        .text("Heeei")
        .build(ctx)
}