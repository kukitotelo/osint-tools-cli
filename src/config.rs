use crossterm::event::{KeyCode, KeyModifiers};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Config {
    pub keybindings: Keybindings,
    pub ui: UiConfig,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Keybindings {
    pub quit: Vec<Key>,
    pub up: Vec<Key>,
    pub down: Vec<Key>,
    pub left: Vec<Key>,
    pub right: Vec<Key>,
    pub enter: Vec<Key>,
    pub back: Vec<Key>,
    pub search: Vec<Key>,
    pub help: Vec<Key>,
    pub home: Vec<Key>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct UiConfig {
    pub show_help: bool,
    pub show_search: bool,
    pub highlight_style: HighlightStyle,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HighlightStyle {
    pub fg: String,
    pub bg: String,
    pub bold: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Key {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            keybindings: Keybindings::default(),
            ui: UiConfig::default(),
        }
    }
}

impl Default for Keybindings {
    fn default() -> Self {
        Self {
            quit: vec![
                Key { code: KeyCode::Char('q'), modifiers: KeyModifiers::NONE },
                Key { code: KeyCode::Esc, modifiers: KeyModifiers::NONE },
                Key { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL },
            ],
            up: vec![
                Key { code: KeyCode::Up, modifiers: KeyModifiers::NONE },
                Key { code: KeyCode::Char('k'), modifiers: KeyModifiers::NONE },
            ],
            down: vec![
                Key { code: KeyCode::Down, modifiers: KeyModifiers::NONE },
                Key { code: KeyCode::Char('j'), modifiers: KeyModifiers::NONE },
            ],
            left: vec![
                Key { code: KeyCode::Left, modifiers: KeyModifiers::NONE },
                Key { code: KeyCode::Char('h'), modifiers: KeyModifiers::NONE },
            ],
            right: vec![
                Key { code: KeyCode::Right, modifiers: KeyModifiers::NONE },
                Key { code: KeyCode::Char('l'), modifiers: KeyModifiers::NONE },
            ],
            enter: vec![
                Key { code: KeyCode::Enter, modifiers: KeyModifiers::NONE },
            ],
            back: vec![
                Key { code: KeyCode::Backspace, modifiers: KeyModifiers::NONE },
                Key { code: KeyCode::Left, modifiers: KeyModifiers::NONE },
            ],
            search: vec![
                Key { code: KeyCode::Char('/'), modifiers: KeyModifiers::NONE },
            ],
            help: vec![
                Key { code: KeyCode::Char('?'), modifiers: KeyModifiers::NONE },
                Key { code: KeyCode::F(1), modifiers: KeyModifiers::NONE },
            ],
            home: vec![
                Key { code: KeyCode::Home, modifiers: KeyModifiers::NONE },
                Key { code: KeyCode::Char('g'), modifiers: KeyModifiers::NONE },
            ],
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            show_help: true,
            show_search: true,
            highlight_style: HighlightStyle::default(),
        }
    }
}

impl Default for HighlightStyle {
    fn default() -> Self {
        Self {
            fg: "white".to_string(),
            bg: "dark_gray".to_string(),
            bold: true,
        }
    }
}
