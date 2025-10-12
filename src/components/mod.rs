pub mod category_list;
pub mod preview_panel;
pub mod search_bar;

use crate::action::Action;
use color_eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::*;
use tokio::sync::mpsc::UnboundedSender;

pub trait Component {
    fn register_action_handler(&mut self, _tx: UnboundedSender<Action>) -> Result<()> {
        Ok(())
    }

    fn handle_key_events(&mut self, _key: KeyEvent) -> Result<Option<Action>> {
        Ok(None)
    }

    fn update(&mut self, _action: Action) -> Result<Option<Action>> {
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()>;
}