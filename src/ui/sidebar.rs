use bevy_ecs::world::World;
use ratatui::{prelude::*, widgets::*};

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

    let player_info = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Name: ", Style::default().fg(Color::Gray)),
            // Span::styled(&player.name, Style::default().fg(Color::Yellow)),
            Span::styled("DUPA", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::styled("Level: ", Style::default().fg(Color::Gray)),
            // Span::styled(player.level.to_string(), Style::default().fg(Color::Green)),
            Span::styled("12", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("HP: ", Style::default().fg(Color::Gray)),
            Span::styled(
                // format!("{}/{}", player.hp, player.max_hp),
                "EEE",
                Style::default().fg(Color::Red),
            ),
        ]),
        Line::from(vec![
            Span::styled("EXP: ", Style::default().fg(Color::Gray)),
            // Span::styled(player.exp.to_string(), Style::default().fg(Color::Blue)),
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
                // player.attack.to_string(),
                "TOMEK",
                Style::default().fg(Color::LightRed),
            ),
        ]),
        Line::from(vec![
            Span::styled("Defense: ", Style::default().fg(Color::Gray)),
            Span::styled(
                "E",
                // player.defense.to_string(),
                Style::default().fg(Color::LightBlue),
            ),
        ]),
        Line::from(vec![
            Span::styled("Position: ", Style::default().fg(Color::Gray)),
            Span::styled(
                // format!("({}, {})", player.position.x, player.position.y),
                "POS",
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

    let turn_info = Paragraph::new(vec![Line::from(vec![
        Span::styled("Turn: ", Style::default().fg(Color::Gray)),
        Span::styled(
            // world.game_state.turn.to_string(),
            "12",
            Style::default().fg(Color::White),
        ),
    ])])
    .block(turn_block);

    f.render_widget(turn_info, chunks[3]);
}
