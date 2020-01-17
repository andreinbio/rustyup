use orbtk::prelude::*;

use crate::prelude::*;

pub fn render_header(id: Entity, ctx: &mut BuildContext, tabs: Vec<Entity>) -> Entity {
    let mut stack: Stack = Stack::create()
        .orientation("horizontal");

    let text: [&str;3] = ["Settings", "Tab 1", "Tab 2"];
    let colors: [&str;3] = ["settings", "tab1", "tab2"];

    for index in  0..3 {
        let tab = tabs[index];
        let pushed_tabs = tabs.clone();
        stack = stack.child(
            Button::create()
                .selector(colors[index])
                .text(text[index])
                .on_click(move |states, _| {
                    states.get_mut::<MainViewState>(id).change_tab(tab, pushed_tabs.clone());
                    true
                })
                .build(ctx)
        );
    }

    stack.build(ctx)
}
