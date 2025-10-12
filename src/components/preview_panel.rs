use crate::{
    action::Action,
    components::Component,
    models::OsintCategory,
};
use color_eyre::Result;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug)]
pub struct PreviewPanel {
    action_tx: Option<UnboundedSender<Action>>,
}

impl Default for PreviewPanel {
    fn default() -> Self {
        Self { action_tx: None }
    }
}

impl PreviewPanel {
    pub fn new() -> Self {
        Self::default()
    }

    fn render_category_preview(&self, f: &mut Frame<'_>, area: Rect, category: &OsintCategory) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        // Description
        let description = Paragraph::new(category.description.as_str())
            .block(
                Block::default()
                    .title("Description")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Green)),
            )
            .wrap(Wrap { trim: true });

        f.render_widget(description, chunks[0]);

        // Content preview
        if !category.subcategories.is_empty() {
            let items: Vec<ListItem> = category
                .subcategories
                .iter()
                .map(|sub| {
                    let tool_count = sub.tools.len();
                    let subcat_count = sub.subcategories.len();
                    let count_info = if tool_count > 0 && subcat_count > 0 {
                        format!(" ({} tools, {} subcategories)", tool_count, subcat_count)
                    } else if tool_count > 0 {
                        format!(" ({} tools)", tool_count)
                    } else if subcat_count > 0 {
                        format!(" ({} subcategories)", subcat_count)
                    } else {
                        String::new()
                    };

                    ListItem::new(vec![
                        Line::from(vec![
                            Span::styled("▶ ", Style::default().fg(Color::Blue)),
                            Span::styled(&sub.name, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                            Span::styled(count_info, Style::default().fg(Color::Gray)),
                        ]),
                        Line::from(vec![
                            Span::styled("   ", Style::default()),
                            Span::styled(&sub.description, Style::default().fg(Color::Gray)),
                        ]),
                    ])
                })
                .collect();

            let preview_list = List::new(items).block(
                Block::default()
                    .title(format!("Subcategories ({})", category.subcategories.len()))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Green)),
            );

            f.render_widget(preview_list, chunks[1]);
        } else if !category.tools.is_empty() {
            let items: Vec<ListItem> = category
                .tools
                .iter()
                .enumerate()
                .map(|(i, tool)| {
                    ListItem::new(vec![
                        Line::from(vec![
                            Span::styled(format!("{}. ", i + 1), Style::default().fg(Color::Yellow)),
                            Span::styled(&tool.name, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                        ]),
                        Line::from(vec![
                            Span::styled("   🔗 ", Style::default().fg(Color::Blue)),
                            Span::styled(&tool.url, Style::default().fg(Color::Blue).add_modifier(Modifier::UNDERLINED)),
                        ]),
                        Line::from(vec![
                            Span::styled("   📝 ", Style::default().fg(Color::Green)),
                            Span::styled(&tool.description, Style::default().fg(Color::White)),
                        ]),
                        Line::from(vec![
                            Span::styled("", Style::default()),
                        ]),
                    ])
                })
                .collect();

            let tools_list = List::new(items).block(
                Block::default()
                    .title(format!("🔧 Tools ({}) - Press Enter to open in browser", category.tools.len()))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Green)),
            );

            f.render_widget(tools_list, chunks[1]);
        } else {
            let empty_msg = Paragraph::new("No subcategories or tools available")
                .block(
                    Block::default()
                        .title("Content")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Green)),
                )
                .style(Style::default().fg(Color::Gray))
                .alignment(Alignment::Center);

            f.render_widget(empty_msg, chunks[1]);
        }
    }
}

impl Component for PreviewPanel {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        // This method needs access to the selected category from CategoryList
        // For now, we'll render a placeholder
        let placeholder = Paragraph::new("Select a category to see preview")
            .block(
                Block::default()
                    .title("Preview")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow)),
            )
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);

        f.render_widget(placeholder, area);
        Ok(())
    }
}

impl PreviewPanel {
    pub fn draw_with_category(&mut self, f: &mut Frame<'_>, area: Rect, category: Option<&OsintCategory>) -> Result<()> {
        match category {
            Some(cat) => self.render_category_preview(f, area, cat),
            None => {
                let no_selection = Paragraph::new("No category selected")
                    .block(
                        Block::default()
                            .title("Preview")
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Yellow)),
                    )
                    .style(Style::default().fg(Color::Gray))
                    .alignment(Alignment::Center);

                f.render_widget(no_selection, area);
            }
        }
        Ok(())
    }
}
