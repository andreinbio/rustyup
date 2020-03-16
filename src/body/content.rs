use orbtk::prelude::*;

use crate::{MainState, Action};

pub fn create_content_body(id: Entity, ctx: &mut BuildContext) -> Grid {
    Grid::create()
        .vertical_alignment(Alignment::Start)
        .clip(true)
        .columns(Columns::create().column(200.0).column("*").build())
        .child(
            get_body_tabs_buttons(id, ctx)
        )
        .child(
            ItemsWidget::create()
                .attach(Grid::column(1))
                .id("body_tabs")
                .items_builder(move |bc, index| {
                    let active_body_tab: usize = bc.get_widget(id).get::<usize>("active_body_tab").clone();

                    if active_body_tab == 0 {
                        get_body_settings_content(id, bc)
                    } else {
                        get_body_data_content(id, bc)
                    }
                })
                .count(1)
                .build(ctx)
        )
}

// buttons to change between body tabs
fn get_body_tabs_buttons(id: Entity, ctx: &mut BuildContext) -> Entity {
    Stack::create()
        .attach(Grid::column(0))
        .vertical_alignment(Alignment::Start)
        .clip(true)
        .child(
            Button::create()
                .text("Settings")
                .on_click(move |states, _| {
                    states.get_mut::<MainState>(id).action(Action::ChangeBodyTab(0));
                    true
                })
                .build(ctx)
        )
        .child(
            Button::create()
                .text("Data")
                .on_click(move |states, _| {
                    states.get_mut::<MainState>(id).action(Action::ChangeBodyTab(1));
                    true
                })
                .build(ctx)
        )
        .build(ctx)
}

// settings content for each tab
fn get_body_settings_content(id: Entity, ctx: &mut BuildContext) -> Entity {
    Stack::create()
        .attach(Grid::column(1))
        .horizontal_alignment(Alignment::Stretch)
        .orientation("horizontal")
        .child(
            TextBlock::create()
                .class("key")
                .text("hostname: ")
                .build(ctx)
        )
        .child(
            TextBox::create()
                .class("value")
                .horizontal_alignment(Alignment::Stretch)
                .water_mark("Text block")
                .text(("text", id))
                .on_activate(move |states, entity| {
                    states.get_mut::<MainState>(id).action(Action::EntryActivated(entity));
                })
                .build(ctx)
        )
        .build(ctx)
}

// data content for body
fn get_body_data_content(id: Entity, ctx: &mut BuildContext) -> Entity {
    Button::create()
        .text("Heeei")
        .build(ctx)
}
