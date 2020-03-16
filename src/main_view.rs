use orbtk::prelude::*;

use crate::{MainState, render_header, render_body, appsettings::*};

type Tabs = Vec<Tab>;

widget!(MainView<MainState> {
    tabs: Tabs,
    active_tab: usize,
    active_body_tab: usize,
    active_settings_tab: usize,
    text: String16
});

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView")
            .child(
                Grid::create()
                    .rows(Rows::create().row(50.0).row("*").build())
                    .child(
                        Grid::create()
                            .attach(Grid::row(0))
                            .child(render_header(id, ctx))
                            .build(ctx)
                    )
                    .child(render_body(id, ctx))
                    .build(ctx)
            )
    }
}
