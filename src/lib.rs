use dpi::{Position, Size};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
#[cfg(windows)]
use tao::window::Theme as TaoTheme;

mod config;
mod image;
mod menu;
mod resources;
mod tray;
pub use crate::config::*;
pub use crate::image::*;
pub use crate::menu::*;
pub use crate::resources::*;
pub use crate::tray::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Icon<'a> {
    /// RGBA bytes of the icon.
    pub rgba: Cow<'a, [u8]>,
    /// Icon width.
    pub width: u32,
    /// Icon height.
    pub height: u32,
}

// -----------------------------------------------------------------------------
// RawWindow
// -----------------------------------------------------------------------------

/// Platform-specific view of a freshly created native window.
///
/// The lifetime is tied directly to the underlying Tao window. This prevents a
/// `RawWindow` from outliving the native window from which its platform handles
/// were obtained.
///
/// A `RawWindow` is intentionally short-lived. It is created for callbacks and
/// native initialization and must not be stored beyond those operations.
pub struct RawWindow<'a> {
    #[cfg(windows)]
    pub hwnd: isize,
    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub gtk_window: &'a gtk::ApplicationWindow,
    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub default_vbox: Option<&'a gtk::Box>,
    pub _marker: &'a std::marker::PhantomData<()>,
}

pub struct WindowMenu {
    pub is_app_wide: bool,
    pub menu: Menu,
}

// -----------------------------------------------------------------------------
// Theme mapping
//
// `muda::MenuTheme` is primarily meaningful on Windows. The mapping functions
// remain available on every platform so callers do not need platform-specific
// cfg gates.
// -----------------------------------------------------------------------------

#[cfg(windows)]
pub fn map_theme_to_muda(theme: Theme) -> muda::MenuTheme {
    match theme {
        Theme::Light => muda::MenuTheme::Light,
        Theme::Dark => muda::MenuTheme::Dark,

        #[allow(unreachable_patterns)]
        _ => muda::MenuTheme::Auto,
    }
}
#[cfg(windows)]
pub fn map_muda_theme_from_tao(theme: TaoTheme) -> muda::MenuTheme {
    match theme {
        TaoTheme::Light => muda::MenuTheme::Light,
        TaoTheme::Dark => muda::MenuTheme::Dark,
        _ => muda::MenuTheme::Auto,
    }
}

/// System theme.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Theme {
    /// Light theme.
    Light,
    /// Dark theme.
    Dark,
}

impl Serialize for Theme {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl<'de> Deserialize<'de> for Theme {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.to_lowercase().as_str() {
            "dark" => Self::Dark,
            _ => Self::Light,
        })
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Light => "light",
                Self::Dark => "dark",
            }
        )
    }
}
/// A rectangular region.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Rect {
    /// Rect position.
    pub position: dpi::Position,
    /// Rect size.
    pub size: dpi::Size,
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            position: Position::Logical((0, 0).into()),
            size: Size::Logical((0, 0).into()),
        }
    }
}

