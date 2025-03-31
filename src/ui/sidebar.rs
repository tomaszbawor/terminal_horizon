use bevy_ecs::{query::With, world::World};
use ratatui::{prelude::*, widgets::*};

use crate::game::{
    components::{Health, Name, Player, Position, Stats},
    state::GameTurn,
};

pub fn render(f: &mut Frame, world: &mut World, area: Rect) {
    // Create blocks for different sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9), // Player info
            Constraint::Length(7), // Stats
            Constraint::Min(3),    // Controls
            Constraint::Length(3), // Turn Number
        ])
        .split(area);

    // Player info block
    let player_block = Block::default()
        .title("Character")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let mut player_props_query = world.query_filtered::<(&Name, &Stats, &Position), With<Player>>();
    let (player_name, stats, ppos) = player_props_query.single(world);

    let player_info = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Name: ", Style::default().fg(Color::Gray)),
            Span::styled(&player_name.0, Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::styled("Level: ", Style::default().fg(Color::Gray)),
            //TODO: Implement
            // Span::styled(player.level.to_string(), Style::default().fg(Color::Green)),
            Span::styled("12", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("HP: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}/{}", stats.health.hp, stats.health.max_hp),
                Style::default().fg(Color::Red),
            ),
        ]),
    ])
    .block(player_block);

    f.render_widget(player_info, chunks[0]);

    // Stats block
    let stats_block = Block::default()
        .title("Stats")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let stats_info = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Attack: ", Style::default().fg(Color::Gray)),
            Span::styled(
                stats.attack.to_string(),
                Style::default().fg(Color::LightRed),
            ),
        ]),
        Line::from(vec![
            Span::styled("Defense: ", Style::default().fg(Color::Gray)),
            Span::styled(
                stats.defense.to_string(),
                Style::default().fg(Color::LightBlue),
            ),
        ]),
        Line::from(vec![
            Span::styled("Position: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("({}, {})", ppos.x, ppos.y),
                Style::default().fg(Color::White),
            ),
        ]),
    ])
    .block(stats_block);

    f.render_widget(stats_info, chunks[1]);

    // Controls block
    let controls_block = Block::default()
        .title("Controls")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let controls_info = Paragraph::new(vec![
        Line::from("Movement: ↑/↓/←/→ or WASD"),
        Line::from("ESC: Return to menu"),
        Line::from("Q: Quit game"),
    ])
    .block(controls_block);

    f.render_widget(controls_info, chunks[2]);

    // Current turn
    let turn_block = Block::default()
        .title("Game")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let turn = world.resource::<GameTurn>();
    let turn_nikm = turn.0;

    let turn_info = Paragraph::new(vec![Line::from(vec![
        Span::styled("Turn: ", Style::default().fg(Color::Gray)),
        Span::styled(turn_nikm.to_string(), Style::default().fg(Color::White)),
    ])])
    .block(turn_block);

    f.render_widget(turn_info, chunks[3]);
}
