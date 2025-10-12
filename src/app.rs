use crate::{
    action::Action,
    components::{
        category_list::CategoryList,
        preview_panel::PreviewPanel,
        Component
    },
};
use color_eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::{prelude::*};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

pub struct App {
    should_quit: bool,
    action_tx: UnboundedSender<Action>,
    action_rx: UnboundedReceiver<Action>,

    // Components
    category_list: CategoryList,
    preview_panel: PreviewPanel,

    // UI state
    focus: Focus,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Focus {
    CategoryList,
}

impl App {
    pub fn new() -> Result<Self> {
        let (action_tx, action_rx) = mpsc::unbounded_channel();

        let mut category_list = CategoryList::new();
        let mut preview_panel = PreviewPanel::new();
        // Register action handlers
        category_list.register_action_handler(action_tx.clone())?;
        preview_panel.register_action_handler(action_tx.clone())?;

        Ok(Self {
            should_quit: false,
            action_tx,
            action_rx,
            category_list,
            preview_panel,
            focus: Focus::CategoryList,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut tui = crate::tui::Tui::new()?;
        tui.enter()?;

        loop {
            if let Some(e) = tui.next().await {
                match e {
                    crate::tui::Event::Quit => self.should_quit = true,
                    crate::tui::Event::Key(key) => {
                        self.handle_key_events(key)?;
                    }
                    crate::tui::Event::Render => {
                        tui.draw(|f| {
                            let _ = self.draw(f);
                        })?;
                    }
                    _ => {}
                }

                // Process action queue
                while let Ok(action) = self.action_rx.try_recv() {
                    match action {
                        Action::Quit => self.should_quit = true,
                        Action::OpenTool(url) => {
                            // Open tool in browser
                            if let Err(e) = Self::open_url_in_browser(&url) {
                                log::error!("Failed to open URL {}: {}", url, e);
                            }
                        }
                        _ => {
                            // Handle actions for each component
                            self.category_list.update(action.clone())?;
                            self.preview_panel.update(action.clone())?;
                        }
                    }
                }

                if self.should_quit {
                    tui.stop()?;
                    break;
                }
            }
        }
        tui.exit()?;
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) -> Result<()> {
        // Split main content into two columns
        let content_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Category list
                Constraint::Percentage(50), // Preview panel
            ])
            .split(frame.area());

        // Render category list
        self.category_list.draw(frame, content_layout[0])?;

        // Render preview panel with selected category
        let selected_category = self.category_list.selected_category();
        self.preview_panel.draw_with_category(frame, content_layout[1], selected_category)?;

        // Draw focus indicator
        match self.focus {
            Focus::CategoryList => {
                let _focused_area = content_layout[0];
                // Add focus styling if needed
            }
        }

        Ok(())
    }

    pub fn handle_key_events(&mut self, key: KeyEvent) -> Result<()> {
        match self.focus {
            Focus::CategoryList => {
                if let Some(action) = self.category_list.handle_key_events(key)? {
                    self.action_tx.send(action)?;
                }
            }
        }

        // Handle global key events
        match key.code {
            crossterm::event::KeyCode::Char('q') => {
                self.action_tx.send(Action::Quit)?;
            }
            _ => {}
        }

        Ok(())
    }

    fn open_url_in_browser(url: &str) -> Result<()> {
        let command = if cfg!(target_os = "windows") {
            "cmd"
        } else if cfg!(target_os = "macos") {
            "open"
        } else {
            "xdg-open"
        };

        let args = if cfg!(target_os = "windows") {
            vec!["/C", "start", url]
        } else {
            vec![url]
        };

        std::process::Command::new(command)
            .args(&args)
            .spawn()
            .map_err(|e| color_eyre::eyre::eyre!("Failed to open browser: {}", e))?;

        Ok(())
    }
}
