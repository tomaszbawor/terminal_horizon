use crate::app::App;
use crate::ui::{app_log, map};
use ratatui::Frame;
use ratatui::layout::{Direction, Layout, Rect};
use ratatui::prelude::Constraint;

pub fn render(f: &mut Frame, app: &App, rect: Rect) {
    // Split screen into sidebar and map areas
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),       // Map takes what stays after logs
            Constraint::Length(5 + 2), // Game logs takes 5 Line plus border
        ])
        .split(rect);

    // Render map
    map::render(f, app, chunks[0]);

    // Render application log
    app_log::render(f, app, chunks[1]);
}
