use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Suspend,
    Resume,
    Quit,
    Refresh,
    Error(String),
    Help,

    // Navigation actions
    Up,
    Down,
    Left,
    Right,
    Enter,
    Back,
    Home,

    // Menu-specific actions
    SelectCategory,
    ShowPreview,
    OpenTool(String), // URL to open
    SearchTool,
    FilterCategory,
}