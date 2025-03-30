use std::cell;

use crate::{
    app::App,
    game::{
        components::{Player, Position, Renderable},
        map::GameMap,
    },
};
use bevy_ecs::{query::With, world::World};
use ratatui::{prelude::*, widgets::*};

pub fn render(f: &mut Frame, world: &mut World, area: Rect) {
    // Create a block for the map
    let map_block = Block::default()
        .title("World Map")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    f.render_widget(map_block.clone(), area);

    // Calculate inner area to draw the map tiles
    let inner_area = map_block.inner(area);

    // Get the map and player
    let map = World::resource::<GameMap>(world);
    let query_filtered = World::query_filtered::<&Position, With<Player>>(world);
    let mut player_position_query = query_filtered;
    let mut enemies_query = world.query::<(&Position, &Renderable)>();

    let player_position = player_position_query.single(world);

    // Viewport Calculation
    let viewport_width = inner_area.width as usize;
    let viewport_height = inner_area.height as usize;
    let start_x = player_position.x.saturating_sub(viewport_width / 2);
    let start_y = player_position.y.saturating_sub(viewport_height / 2);
    let end_x = std::cmp::min(start_x + viewport_width, map.width);
    let end_y = std::cmp::min(start_y + viewport_height, map.height);

    // Render map tiles
    for y in start_y..end_y {
        for x in start_x..end_x {
            // Calculate screen position
            let screen_x = inner_area.x + (x - start_x) as u16;
            let screen_y = inner_area.y + (y - start_y) as u16;

            // Skip if outside screen bounds
            if screen_x >= inner_area.right() || screen_y >= inner_area.bottom() {
                continue;
            }

            // Default with map tiles
            let symbol = map.get_tile_symbol(x, y);
            let style = map.get_tile_style(x, y);

            if let Some(cell) = f
                .buffer_mut()
                .cell_mut(ratatui::prelude::Position::new(screen_x, screen_y))
            {
                cell.set_symbol(symbol);
                cell.set_style(style);
            };

            // Render Entities on Map
            //
            for (pos, renderable) in enemies_query.iter(world) {}

            // Override if player
            // if x == player_position.x && y == player_position.y {
            //     symbol = "󰋦";
            //     style = Style::default().fg(Color::Yellow)
            // }
            //
            // // Override if enemy
            // if let Some(enemy) = enemies
            //     .iter()
            //     .find(|&e| e.position.x == x && e.position.y == y)
            // {
            //     symbol = &enemy.symbol;
            //     style = Style::default().fg(Color::LightRed); // enemy color
            // }
            //
            // // Render the tile at the calculated position
            // if let Some(cell) = f.buffer_mut().cell_mut(Position::new(screen_x, screen_y)) {
            //     cell.set_style(style).set_symbol(symbol);
            // }
        }
    }
}
