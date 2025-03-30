use bevy_ecs::system::Resource;

use super::ActionLog;
use super::entities::Enemy;
use super::map::GameMap;
use super::player::Player;

#[derive(Clone)]
pub struct GameState {
    pub player: Player,
    pub map: GameMap,
    pub enemies: Vec<Enemy>,
    pub journal: Vec<ActionLog>,
    pub turn: u32,
}

#[derive(Resource, Default)]
pub struct ActionJournal {
    pub entries: Vec<ActionLog>,
}

#[derive(Resource, Default)]
pub struct GameTurn(pub u32);
