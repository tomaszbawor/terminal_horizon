use bevy_ecs::system::Resource;

use super::ActionLog;

#[derive(Resource, Default)]
pub struct ActionJournal {
    pub entries: Vec<ActionLog>,
}

#[derive(Resource, Default)]
pub struct GameTurn(pub u32);

#[derive(Resource, Default)]
pub struct ActiveMenuIndex(pub usize);
