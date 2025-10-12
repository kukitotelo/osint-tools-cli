use crate::{
    action::Action,
    components::Component,
    models::{OsintCategory, create_osint_categories},
};
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState},
};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug)]
pub struct CategoryList {
    action_tx: Option<UnboundedSender<Action>>,
    categories: Vec<OsintCategory>,
    current_categories: Vec<OsintCategory>,
    state: ListState,
    navigation_stack: Vec<Vec<OsintCategory>>,
    current_path: Vec<String>,
}

impl Default for CategoryList {
    fn default() -> Self {
        let categories = create_osint_categories();
        let current_categories = categories.clone();

        let mut state = ListState::default();
        if !current_categories.is_empty() {
            state.select(Some(0));
        }

        Self {
            action_tx: None,
            categories,
            current_categories,
            state,
            navigation_stack: Vec::new(),
            current_path: Vec::new(),
        }
    }
}

impl CategoryList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn selected_category(&self) -> Option<&OsintCategory> {
        self.state.selected().and_then(|i| self.current_categories.get(i))
    }

    #[allow(dead_code)]
    pub fn current_path(&self) -> &[String] {
        &self.current_path
    }

    fn select_next(&mut self) {
        if self.current_categories.is_empty() {
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.current_categories.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn select_previous(&mut self) {
        if self.current_categories.is_empty() {
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.current_categories.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn enter_category(&mut self) -> Result<Option<Action>> {
        if let Some(selected_idx) = self.state.selected() {
            if selected_idx < self.current_categories.len() {
                let category = &self.current_categories[selected_idx];
                let category_name = category.name.clone();
                let subcategories = category.subcategories.clone();
                let tools = &category.tools;

                if !subcategories.is_empty() {
                    // Navigate into subcategory
                    self.navigation_stack.push(self.current_categories.clone());
                    self.current_categories = subcategories;
                    self.current_path.push(category_name);
                    self.state.select(Some(0));

                    if let Some(tx) = &self.action_tx {
                        let _ = tx.send(Action::ShowPreview);
                    }
                } else if !tools.is_empty() {
                    // If it's a leaf category with only one tool, open it directly
                    if tools.len() == 1 {
                        return Ok(Some(Action::OpenTool(tools[0].url.clone())));
                    } else {
                        // Multiple tools - let user navigate to tool list
                        self.navigation_stack.push(self.current_categories.clone());
                        self.current_categories = vec![];
                        self.current_path.push(category_name);
                        self.state.select(Some(0));

                        if let Some(tx) = &self.action_tx {
                            let _ = tx.send(Action::ShowPreview);
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    fn go_back(&mut self) -> Result<Option<Action>> {
        if let Some(previous_categories) = self.navigation_stack.pop() {
            self.current_categories = previous_categories;
            self.current_path.pop();
            self.state.select(Some(0));

            if let Some(tx) = &self.action_tx {
                let _ = tx.send(Action::ShowPreview);
            }
        } else if let Some(tx) = &self.action_tx {
            let _ = tx.send(Action::Quit);
        }
        Ok(None)
    }
}

impl Component for CategoryList {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.select_previous();
                if let Some(tx) = &self.action_tx {
                    let _ = tx.send(Action::ShowPreview);
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.select_next();
                if let Some(tx) = &self.action_tx {
                    let _ = tx.send(Action::ShowPreview);
                }
            }
            KeyCode::Enter => {
                return self.enter_category();
            }
            KeyCode::Left | KeyCode::Char('h') | KeyCode::Backspace => {
                return self.go_back();
            }
            KeyCode::Char('q') | KeyCode::Esc => {
                return self.go_back();
            }
            _ => {}
        }
        Ok(None)
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Up => {
                self.select_previous();
            }
            Action::Down => {
                self.select_next();
            }
            Action::Enter => {
                return self.enter_category();
            }
            Action::Back => {
                return self.go_back();
            }
            Action::Home => {
                self.navigation_stack.clear();
                self.current_categories = self.categories.clone();
                self.current_path.clear();
                self.state.select(Some(0));
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let title = if self.current_path.is_empty() {
            "OSINT Tools Categories".to_string()
        } else {
            format!("OSINT Tools - {}", self.current_path.join(" > "))
        };

        let items: Vec<ListItem> = self
            .current_categories
            .iter()
            .map(|category| {
                let indicator = if category.has_children() { " >" } else { "" };
                ListItem::new(format!("{}{}", category.name, indicator))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Blue)),
            )
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::DarkGray)
                    .fg(Color::White),
            )
            .highlight_symbol("→ ");

        f.render_stateful_widget(list, area, &mut self.state);
        Ok(())
    }
}
