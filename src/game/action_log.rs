#[derive(Debug, Clone)]
pub struct ActionLog {
    pub turn: u32,
    pub action_type: ActionType,
}

#[derive(Debug, Clone)]
pub enum ActionType {
    Movement { x: usize, y: usize },
}

impl ActionLog {
    pub fn new(turn: u32, action: ActionType) -> Self {
        Self {
            turn,
            action_type: action,
        }
    }
}
