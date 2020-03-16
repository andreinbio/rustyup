use orbtk::prelude::*;

use crate::update;
use crate::{appsettings, appsettings::*};

#[derive(Copy, Clone)]
pub enum Action {
    AddTab,
    RemoveTab(usize),
    ChangeTab(usize),
    ChangeBodyTab(usize),
    ChangeSettingsTab(usize),
    EntryActivated(Entity),
}

#[derive(AsAny)]
pub struct MainState {
    action: Option<Action>,
}

impl Default for MainState {
    fn default() -> Self {
        MainState { action: None }
    }
}

impl MainState {
    pub fn action(&mut self, action: impl Into<Option<Action>>) {
        self.action = action.into();
    }
}

impl State for MainState {
    fn init(&mut self, registry: &mut Registry, ctx: &mut Context<'_>) {
        // load saved settings and populate every item
        let app_settings: appsettings::Global = load_app_settings(registry, ctx);
        // ctx.widget().get_mut::<Vec<String>>("tabs").push("Settings".to_owned());// @TOTOD get the tabs from settings
        *ctx.widget().get_mut::<Vec<Tab>>("tabs") = app_settings.get_tabs();
        *ctx.widget().get_mut::<usize>("active_tab") = 0;
        *ctx.widget().get_mut::<usize>("active_body_tab") = 0;
        *ctx.widget().get_mut::<usize>("active_settings_tab") = 0;
    }

    fn update(&mut self, registry: &mut Registry, ctx: &mut Context<'_>) {

        if let Some(action) = self.action {
            match action {
                Action::AddTab => {
                    let tabs_count = ctx.widget().get::<Vec<Tab>>("tabs").len();
                    ctx.widget().get_mut::<Vec<Tab>>("tabs").push(Tab::new("Tab"));
                    ctx.child("header_tabs").set("request_update", true);
                    *ctx.widget().get_mut::<usize>("active_tab") = tabs_count;
                    ctx.child("body_content").set("request_update", true);
                },
                Action::RemoveTab(index) => {
                    let tabs_count = ctx.widget().get::<Vec<Tab>>("tabs").len();
                    let active_tab = ctx.widget().get_mut::<usize>("active_tab").clone();
                    ctx.widget().get_mut::<Vec<Tab>>("tabs").remove(index);
                    ctx.child("header_tabs").set("request_update", true);
                    *ctx.widget().get_mut::<usize>("active_tab") = if active_tab > 0 {
                        active_tab - 1
                    } else {
                        0
                    };
                    ctx.child("body_content").set("request_update", true);
                },
                Action::ChangeTab(index) => {
                    *ctx.widget().get_mut::<usize>("active_tab") = index;
                    ctx.child("body_content").set("request_update", true);
                },
                Action::ChangeBodyTab(index) => {
                    *ctx.widget().get_mut::<usize>("active_body_tab") = index;
                    ctx.child("body_tabs").set("request_update", true);
                },
                Action::ChangeSettingsTab(index) => {
                    *ctx.widget().get_mut::<usize>("active_settings_tab") = index;
                    ctx.child("settings_tabs").set("request_update", true);
                },
                Action::EntryActivated(entity) => {
                    let mut widget = ctx.get_widget(entity);
                    let text = widget.get_mut::<String16>("text");
                    text.clear();
                },
            }

            self.action = None;
        }
    }
}