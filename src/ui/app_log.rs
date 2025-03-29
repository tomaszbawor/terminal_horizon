use crate::app::App;
use crate::game::action_log::ActionLog;
use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let action_to_show: Vec<ActionLog> = app
        .game_state
        .journal
        .iter()
        .rev()
        .take(5)
        .cloned()
        .collect();

    let log_lines: Vec<String> = action_to_show.iter().map(create_log_entry).collect();

    let log_block = Block::default()
        .title("Action Journal")
        .title_alignment(Alignment::Right)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let text_log: Vec<ListItem> = log_lines
        .iter()
        .map(|log_line| ListItem::from(log_line.clone()))
        .collect();

    let list = List::new(text_log).block(log_block);
    f.render_widget(list, area);
}

fn create_log_entry(action_log: &ActionLog) -> String {
    let log_message = match action_log.action_type {
        crate::game::action_log::ActionType::Movement { x, y } => {
            format!("Player moved into: ({}, {})", x, y)
        }
    };

    format!("[Turn: {}]: {}", action_log.turn, log_message)
}
