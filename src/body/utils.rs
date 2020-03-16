use orbtk::prelude::*;

use crate::{MainState, Action};

pub fn create_button_row(id: Entity, ctx: &mut BuildContext, text: &str) -> Entity {
    Stack::create()
        // .horizontal_alignment(Alignment::Stretch)
        .orientation("horizontal")
        .child(
            TextBlock::create()
                .class("key")
                .text(format!("{0}: ", text))
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

pub fn create_cartridges_data(id: Entity, ctx: &mut BuildContext, text: &str) -> Entity {
    Stack::create()
        .child(
            TextBlock::create()
                .text(text)
                .build(ctx)
        )
        .child(
            ItemsWidget::create()
                // .items_builder(move |bc, index| {
                //
                // })
                .build(ctx)
        )
        .build(ctx)
}
