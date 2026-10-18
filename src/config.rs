// taurino-config/src/menu.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PredefinedAction {
    Separator,
    Copy,
    Cut,
    Paste,
    SelectAll,
    Undo,
    Redo,
    Minimize,
    Maximize,
    Fullscreen,
    Hide,
    HideOthers,
    ShowAll,
    CloseWindow,
    Quit,
    Services,
    BringAllToFront,
    About,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConfig {
    pub kind: String,
    pub path: Option<String>,
    pub data: Option<Vec<u8>>,
    pub rgba: Option<Vec<u8>>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AboutMetadataConfig {
    pub name: Option<String>,
    pub version: Option<String>,
    pub short_version: Option<String>,
    pub authors: Option<Vec<String>>,
    pub comments: Option<String>,
    pub copyright: Option<String>,
    pub license: Option<String>,
    pub website: Option<String>,
    pub website_label: Option<String>,
    pub credits: Option<String>,
    pub icon: Option<ImageConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MenuItemConfig {
    Text {
        id: Option<String>,
        text: String,
        enabled: bool,
        accelerator: Option<String>,
    },
    Check {
        id: Option<String>,
        text: String,
        enabled: bool,
        checked: bool,
        accelerator: Option<String>,
    },
    Icon {
        id: Option<String>,
        text: String,
        enabled: bool,
        icon: Option<ImageConfig>,
        native_icon: Option<String>,
        accelerator: Option<String>,
    },
    Predefined {
        action: PredefinedAction,
        metadata: Option<AboutMetadataConfig>,
    },
    Submenu {
        id: Option<String>,
        text: String,
        enabled: bool,
        icon: Option<ImageConfig>,
        native_icon: Option<String>,
        items: Vec<MenuItemConfig>,
    },
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MenuConfig {
    pub id: Option<String>,
    #[serde(default)]
    pub items: Vec<MenuItemConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrayIconConfig {
    pub id: String,
    pub menu: Option<MenuConfig>,
    pub icon: Option<ImageConfig>,
    pub tooltip: Option<String>,
    pub title: Option<String>,
    pub temp_dir_path: Option<String>,
    #[serde(default)]
    pub icon_as_template: bool,
    #[serde(default = "default_true")]
    pub show_menu_on_left_click: bool,
    #[serde(default = "default_true")]
    pub visible: bool,
}

fn default_true() -> bool {
    true
}
