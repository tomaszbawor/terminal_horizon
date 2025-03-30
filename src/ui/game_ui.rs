use super::game_area;
use super::sidebar;
use bevy_ecs::world::World;
use ratatui::prelude::*;

pub fn render(f: &mut Frame, world: &World) {
    let size = f.area();

    // Split screen into sidebar and map areas
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25), // Sidebar takes 25% of width
            Constraint::Percentage(75), // Map takes 75% of width
        ])
        .split(size);

    // Render sidebar with player stats
    sidebar::render(f, world, chunks[0]);

    // Render map
    game_area::render(f, world, chunks[1]);
}
