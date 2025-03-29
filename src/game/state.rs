use super::map::GameMap;
use super::player::Player;

pub struct GameState {
    pub player: Player,
    pub map: GameMap,
    pub turn: u32,
}
