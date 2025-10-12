use crate::{action::Action, components::Component};
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug)]
pub struct SearchBar {
    action_tx: Option<UnboundedSender<Action>>,
    query: String,
    is_active: bool,
    cursor_position: usize,
}

impl Default for SearchBar {
    fn default() -> Self {
        Self {
            action_tx: None,
            query: String::new(),
            is_active: false,
            cursor_position: 0,
        }
    }
}

impl SearchBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    #[allow(dead_code)]
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    #[allow(dead_code)]
    pub fn query(&self) -> &str {
        &self.query
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.query.clear();
        self.cursor_position = 0;
    }

    fn enter_char(&mut self, c: char) {
        self.query.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.query.remove(self.cursor_position);
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_position < self.query.len() {
            self.cursor_position += 1;
        }
    }
}

impl Component for SearchBar {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        if !self.is_active {
            return Ok(None);
        }

        match key.code {
            KeyCode::Enter => {
                if let Some(tx) = &self.action_tx {
                    let _ = tx.send(Action::SearchTool);
                }
                self.deactivate();
            }
            KeyCode::Esc => {
                self.deactivate();
            }
            KeyCode::Char(c) => {
                self.enter_char(c);
            }
            KeyCode::Backspace => {
                self.delete_char();
            }
            KeyCode::Left => {
                self.move_cursor_left();
            }
            KeyCode::Right => {
                self.move_cursor_right();
            }
            KeyCode::Home => {
                self.cursor_position = 0;
            }
            KeyCode::End => {
                self.cursor_position = self.query.len();
            }
            _ => {}
        }
        Ok(None)
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::SearchTool => {
                self.activate();
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let title = if self.is_active {
            "Search (ESC to cancel)"
        } else {
            "Search (/ to activate)"
        };

        let border_style = if self.is_active {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Gray)
        };

        let display_text = if self.is_active {
            let mut text = self.query.clone();
            if self.cursor_position <= text.len() {
                text.insert(self.cursor_position, '|');
            }
            text
        } else {
            "Type / to search...".to_string()
        };

        let search_input = Paragraph::new(display_text)
            .block(
                Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .border_style(border_style),
            )
            .style(if self.is_active {
                Style::default().fg(Color::White)
            } else {
                Style::default().fg(Color::Gray)
            });

        f.render_widget(search_input, area);
        Ok(())
    }
}