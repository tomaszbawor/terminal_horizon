use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let player = &app.game_state.player;

    // Create blocks for different sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9), // Player info
            Constraint::Length(7), // Stats
            Constraint::Min(3),    // Controls
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
            Span::styled(&player.name, Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::styled("Level: ", Style::default().fg(Color::Gray)),
            Span::styled(player.level.to_string(), Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("HP: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}/{}", player.hp, player.max_hp),
                Style::default().fg(Color::Red),
            ),
        ]),
        Line::from(vec![
            Span::styled("EXP: ", Style::default().fg(Color::Gray)),
            Span::styled(player.exp.to_string(), Style::default().fg(Color::Blue)),
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
                player.attack.to_string(),
                Style::default().fg(Color::LightRed),
            ),
        ]),
        Line::from(vec![
            Span::styled("Defense: ", Style::default().fg(Color::Gray)),
            Span::styled(
                player.defense.to_string(),
                Style::default().fg(Color::LightBlue),
            ),
        ]),
        Line::from(vec![
            Span::styled("Position: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("({}, {})", player.x, player.y),
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
            app.game_state.turn.to_string(),
            Style::default().fg(Color::White),
        ),
    ])])
    .block(turn_block);

    // Get area below the controls
    let turn_area = Rect {
        x: chunks[2].x,
        y: chunks[2].y + chunks[2].height,
        width: chunks[2].width,
        height: 3,
    };

    if turn_area.y + turn_area.height <= area.height {
        f.render_widget(turn_info, turn_area);
    }
}
