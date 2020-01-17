use orbtk::prelude::*;

#[derive(Default, AsAny)]
pub struct MainViewState {
    change_tab: bool,
    active_tab: Entity,
    tabs: Vec<Entity>,
}

impl MainViewState {
    pub fn change_tab(&mut self, tab: Entity, tabs: Vec<Entity>) {
        self.active_tab = tab;
        self.change_tab = true;
        self.tabs = tabs;
    }
}

impl State for MainViewState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if self.change_tab {
            ctx.push_event(SystemEvent::Quit);
            self.change_tab = false;

            for tab in self.tabs.iter() {
                let mut widget = ctx.get_widget(Entity::from(*tab));

                if *tab == self.active_tab {
                    widget.set("visibility", Visibility::Visible);
                } else {
                    widget.set("visibility", Visibility::Collapsed);
                }
            }
        }
    }
}