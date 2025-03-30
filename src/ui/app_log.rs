use crate::app::App;
use crate::game::action_log::ActionLog;
use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let journal_lines: Vec<String> = app
        .game_state
        .journal
        .iter()
        .rev()
        .take(5)
        .map(create_log_entry)
        .collect();

    let journal_ui_block = Block::default()
        .title("Action Journal")
        .title_alignment(Alignment::Right)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let journal_ui_entries: Vec<ListItem> = journal_lines
        .iter()
        .map(|log_line| ListItem::from(log_line.clone()))
        .collect();

    let journal_entries_widget = List::new(journal_ui_entries).block(journal_ui_block);
    f.render_widget(journal_entries_widget, area);
}

fn create_log_entry(action_log: &ActionLog) -> String {
    let log_message = match &action_log.action_type {
        crate::game::action_log::ActionType::Movement { x, y } => {
            format!("Player moved into: ({}, {})", x, y)
        }
        crate::game::action_log::ActionType::MonsterAttack {
            attacker_name,
            target_name,
            damage,
        } => format!(
            "{} attacked {} for {} damage.",
            attacker_name, target_name, damage
        ),
    };

    format!("[Turn: {}]: {}", action_log.turn, log_message)
}
