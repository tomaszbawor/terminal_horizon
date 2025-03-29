use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub fn render(f: &mut Frame, app: &App) {
    let size = f.area();

    // Create a centered block for the menu
    let block = Block::default()
        .title("Terminal Horizon")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    f.render_widget(block, size);

    // Calculate layout for menu items
    let menu_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Length(app.menu_items.len() as u16 * 3),
            Constraint::Percentage(30),
        ])
        .split(size)[1];

    // Create menu items
    let menu_items: Vec<ListItem> = app
        .menu_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.menu_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let text = Text::raw(item).style(style).alignment(Alignment::Center);

            ListItem::new(text)
        })
        .collect();

    // Create and render the menu list
    let menu = List::new(menu_items)
        .block(Block::default().borders(Borders::NONE))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(menu, menu_area);

    // Add some game title and footer
    let title = Paragraph::new("Terminal Horizon")
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);

    let title_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(15), Constraint::Percentage(85)])
        .split(size)[0];

    f.render_widget(title, title_area);

    let footer = Paragraph::new("Use ↑/↓ to navigate and Enter to select. Press 'q' to quit.")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);

    let footer_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(size)[1];

    f.render_widget(footer, footer_area);
}
