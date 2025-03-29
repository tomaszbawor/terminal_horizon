
use super::player::Player;
use super::map::GameMap;

pub struct GameState {
    pub player: Player,
    pub map: GameMap,
    pub turn: u32,
}
