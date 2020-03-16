use orbtk::prelude::*;

use crate::{MainState, Action};
use crate::appsettings::*;

pub fn render_header(id: Entity, ctx: &mut BuildContext) -> Entity {
    ItemsWidget::create()
        .orientation("horizontal")
        .id("header_tabs")
        .items_builder(move |bc, index| {
            get_buttons_entity(id, bc)
        })
        .count(1)
        .build(ctx)
}

fn create_settings_button(id: Entity, ctx: &mut BuildContext, index: usize) -> Entity {
    Button::create()
        .id("settings")
        .text("Settings")
        .on_click(move |states, _| {
            states.get_mut::<MainState>(id).action(Action::ChangeTab(index));
            true
        })
        .build(ctx)
}

fn create_tab_button(id: Entity, ctx: &mut BuildContext, index: usize) -> Entity {
    let tab: Tab = ctx.get_widget(id).get::<Vec<Tab>>("tabs")[index].clone();
    Grid::create()
        .child(
            Button::create()
                .element("tab")
                .text(tab.get_name())
                .on_click(move |states, _| {
                    states.get_mut::<MainState>(id).action(Action::ChangeTab(index));
                    true
                })
                .build(ctx)
        )
        .child(
            Button::create()
                .class("single_content")
                .icon(material_font_icons::MINUS_FONT_ICON)
                .horizontal_alignment(Alignment::End)
                .max_size(20.0, 20.0)
                .on_click(move |states, _| {
                    states.get_mut::<MainState>(id).action(Action::RemoveTab(index));
                    true
                })
                .build(ctx)
        )
        .build(ctx)
}

fn create_add_button(id: Entity, ctx: &mut BuildContext) -> Entity {
    Button::create()
        .horizontal_alignment(Alignment::Start)
        .max_width(40.0)
        .class("single_content")
        .margin((10.0, 0.0, 0.0, 0.0))
        .icon(material_font_icons::ADD_FONT_ICON)
        .on_click(move |states, _| {
            states.get_mut::<MainState>(id).action(Action::AddTab);
            true
        })
        .build(ctx)
}

fn get_buttons_entity(id: Entity, ctx: &mut BuildContext) -> Entity {
    let tabs_count: usize = ctx.get_widget(id).get::<Vec<Tab>>("tabs").len();
    let mut tabs: Stack = Stack::create()
        .orientation("horizontal");

    for tab_index in 0..tabs_count {
        let button_entity: Entity = if tab_index == 0 {
            create_settings_button(id, ctx, tab_index)
        } else {
            create_tab_button(id, ctx, tab_index)
        };

        tabs = tabs.child(
            button_entity
        );
    }

    // add + button to craete a new tab
    tabs = tabs.child(
        create_add_button(id, ctx)
    );

    tabs.build(ctx)
}