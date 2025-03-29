use crate::app::App;
use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem};

pub fn render(f: &mut Frame, _app: &App, area: Rect) {
    // will be taken from the app state
    let log_lines = ["Action One", "Action Two", "Action Three", "Action Four"];

    let log_block = Block::default()
        .title("Action Log")
        .title_alignment(Alignment::Right)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let text_log: Vec<ListItem> = log_lines
        .iter()
        .map(|&log_line| ListItem::from(log_line))
        .collect();

    let list = List::new(text_log).block(log_block);
    f.render_widget(list, area);
}
