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
