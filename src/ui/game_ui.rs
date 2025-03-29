use ratatui::{prelude::*, widgets::*};
use crate::app::App;
use super::sidebar;
use super::map;

pub fn render(f: &mut Frame, app: &App) {
    let size = f.size();
    
    // Split screen into sidebar and map areas
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),  // Sidebar takes 25% of width
            Constraint::Percentage(75),  // Map takes 75% of width
        ])
        .split(size);
    
    // Render sidebar with player stats
    sidebar::render(f, app, chunks[0]);
    
    // Render map
    map::render(f, app, chunks[1]);
}

