pub struct Entity {
    pub x: usize,
    pub y: usize,
    pub symbol: String,
    pub name: String,
}
#[derive(Clone, PartialEq, Debug)]
pub enum AiState {
    Idle,
    Chasing,
}

#[derive(Debug, Clone)]
pub struct Enemy {
    pub x: usize,
    pub y: usize,
    pub symbol: String,
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub ai_state: AiState,
    pub fov_radius: i32, // Field of View radius
                         // Potentially add pathfinding target, last known player position, etc.
}

// Basic movement logic for Enemy - can be expanded in ai.rs later
impl Enemy {
    // Add basic methods like `new`, `move_towards`, etc. later
    pub fn new(
        x: usize,
        y: usize,
        name: &str,
        symbol: &str,
        hp: i32,
        attack: i32,
        defense: i32,
        fov_radius: i32,
    ) -> Self {
        Self {
            x,
            y,
            name: name.to_string(),
            symbol: symbol.to_string(),
            hp,
            max_hp: hp,
            attack,
            defense,
            ai_state: AiState::Idle,
            fov_radius,
        }
    }

    // Simple check for now, replace with proper FOV later
    pub fn is_player_in_fov(&self, player_x: usize, player_y: usize) -> bool {
        let dx = (self.x as i32 - player_x as i32).abs();
        let dy = (self.y as i32 - player_y as i32).abs();
        // Simple distance check for now
        dx <= self.fov_radius && dy <= self.fov_radius
    }
}
