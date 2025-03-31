use bevy_ecs::prelude::*;

use crate::input::handlers::Direction;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Component)]
pub struct Renderable {
    pub symbol: String,
    pub fg: ratatui::style::Color,
    pub bg: ratatui::style::Color,
}

#[derive(Component)]
pub struct Name(pub String);

#[derive(Debug, Component, Clone)]
pub struct Health {
    pub hp: usize,
    pub max_hp: usize,
}
impl Health {
    pub fn new(hp: usize) -> Self {
        Self { hp, max_hp: hp }
    }
}

#[derive(Component)]
pub struct Stats {
    pub health: Health,
    pub attack: u32,
    pub defense: u32,
}

// Tag Components

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct BlocksTile; // Marker for entities that prevent movement

#[derive(Component)]
pub enum AiState {
    Idle,
    Chasing,
}

#[derive(Component)]
pub struct BasicAi {
    pub state: AiState,
    pub target_visible: bool,
    pub last_known_player_pos: Option<Position>,
    pub fov_radius: u32,
}

// --- Action/Intent Components ---
#[derive(Component)]
pub struct WantsToMove {
    pub direction: Direction,
}

#[derive(Component)]
pub struct WantsToAttack {
    pub target: Entity,
}
