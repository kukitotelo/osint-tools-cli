use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Quit,
    Error(String),

    // Navigation actions
    Up,
    Down,
    Left,
    Right,
    Enter,
    Back,
    Home,

    // Menu-specific actions
    ShowPreview,
    OpenTool(String), // URL to open
}
