use serde_derive::{Deserialize, Serialize};
use orbtk::prelude::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Data {
    hostname: String,
    username: String,
    password: String,
    cartridge: Vec<String>,
    code_version: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Tab {
    name: String,
    data: Option<Data>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Global {
    tabs: Vec<Tab>,
}

impl Global {
    pub fn get_tabs(self) -> Vec<Tab> {
        self.tabs.clone()
    }
}

impl Tab {
    pub fn new(name: &str) -> Self {
        Tab {
            name: name.to_owned(),
            data: None,
        }
    }

    pub fn get_name(self) -> String {
        self.name.clone()
    }
}

pub fn load_app_settings(registry: &mut Registry, ctx: &mut Context<'_>) -> Global {
    let app_settings: Global = if let Ok(global) = registry.get::<Settings>("settings").load::<Global>("global") {
        // return saved data
        global
    } else {
        // return default values
        let tab = Tab {
            name: "Settings".to_owned(),
            data: None,
        };

        Global {
            tabs: vec![tab],
        }
    };

    app_settings
}



// struct Editor {
//     tabs: Vec<u32>,
// }
//
// pub struct AppSettings {
//     settings: Settings,
//     editor: Result<Editor, String>,
//     // pub
// }
//
// impl AppSettings {
//     pub fn init(name: &str) -> Self {
//         let settings = Settings::new(name);
//         // settings.save("global", &Global {label: "Ho ho ho".to_owned()});
//         let editor = settings.load::<Editor>("global");
//
//         Self {
//             settings: settings,
//             editor: editor,
//         }
//     }
//
//     pub fn save(&mut self) {
//         // self.settings.save
//     }
//
//     pub fn get_tabs(&self) -> Vec<u32> {
//         if let Ok(editor) = &self.editor {
//             return editor.tabs.clone();
//         }
//
//         vec![]
//     }
// }
