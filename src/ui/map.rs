use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
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
    let map = &app.game_state.map;
    let player = &app.game_state.player;
    let enemies = &app.game_state.enemies;

    // Calculate viewport - center on player
    let viewport_width = inner_area.width as usize;
    let viewport_height = inner_area.height as usize;

    let start_x = player.position.x.saturating_sub(viewport_width / 2);
    let start_y = player.position.y.saturating_sub(viewport_height / 2);

    let end_x = std::cmp::min(start_x + viewport_width, map.width);
    let end_y = std::cmp::min(start_y + viewport_height, map.height);

    // Render map tiles
    for y in start_y..end_y {
        for x in start_x..end_x {
            // Calculate screen position
            let screen_x = inner_area.x + (x - start_x) as u16;
            let screen_y = inner_area.y + (y - start_y) as u16;

            // Skip if outside screen bounds
            if screen_x >= inner_area.x + inner_area.width
                || screen_y >= inner_area.y + inner_area.height
            {
                continue;
            }

            // Default with map tiles
            let mut symbol = map.get_tile_symbol(x, y);
            let mut style: Style = match map.tiles[y][x] {
                crate::game::map::Tile::Floor => Style::default().fg(Color::DarkGray),
                crate::game::map::Tile::Wall => Style::default().fg(Color::White),
                crate::game::map::Tile::Door => Style::default().fg(Color::LightYellow),
                crate::game::map::Tile::Water => Style::default().fg(Color::Blue),
            };

            // Override if player
            if x == player.position.x && y == player.position.y {
                symbol = "󰋦";
                style = Style::default().fg(Color::Yellow)
            }

            // Override if enemy
            if let Some(enemy) = enemies
                .iter()
                .find(|&e| e.position.x == x && e.position.y == y)
            {
                symbol = &enemy.symbol;
                style = Style::default().fg(Color::LightRed); // enemy color
            }

            // Render the tile at the calculated position
            if let Some(cell) = f.buffer_mut().cell_mut(Position::new(screen_x, screen_y)) {
                cell.set_style(style).set_symbol(symbol);
            }
        }
    }
}
