use std::fmt::Debug;

use super::entities::EntityPosition;
use crate::game::state::GameState;
use rand::Rng;

// Represents a potential action an AI can take
#[derive(Debug, Clone, PartialEq)]
pub enum AiAction {
    Wait,
    MoveTo(EntityPosition),
    Attack(usize), // Target entity index/ID (e.g., player ID 0)
}

// Trait for any AI behavior
pub trait AiBehavior: Debug {
    fn decide_next_action(
        &mut self,
        current_pos: &EntityPosition,
        game_state: &GameState,
    ) -> AiAction;

    fn clone_box(&self) -> Box<dyn AiBehavior>;
}

// --- Example: Simple Chasing AI ---
#[derive(Debug, Clone, PartialEq)]
pub enum AiState {
    Idle,
    Chasing,
}

#[derive(Debug, Clone)]
pub struct BasicMonsterAI {
    pub state: AiState,
    pub target_visible: bool, // Track if player is currently visible
    pub last_known_player_pos: Option<EntityPosition>,
    pub fov_radius: i32,
}

impl BasicMonsterAI {
    pub fn new(fov_radius: i32) -> Self {
        Self {
            state: AiState::Idle,
            target_visible: false,
            last_known_player_pos: None,
            fov_radius,
        }
    }

    // Simple distance check (replace with proper FOV later)
    fn is_player_in_fov(&self, self_pos: &EntityPosition, player_pos: &EntityPosition) -> bool {
        let dx = (self_pos.x as i32 - player_pos.x as i32).abs();
        let dy = (self_pos.y as i32 - player_pos.y as i32).abs();
        dx <= self.fov_radius && dy <= self.fov_radius
    }

    // Basic pathfinding (move one step towards target)
    fn move_towards(
        &self,
        current_pos: &EntityPosition,
        target_pos: &EntityPosition,
    ) -> EntityPosition {
        let dx = target_pos.x as i32 - current_pos.x as i32;
        let dy = target_pos.y as i32 - current_pos.y as i32;

        let mut next_x = current_pos.x;
        let mut next_y = current_pos.y;

        // Move horizontally or vertically towards the target
        if dx.abs() > dy.abs() {
            match dx.cmp(&0) {
                std::cmp::Ordering::Less => next_x = next_x.saturating_sub(1),
                std::cmp::Ordering::Greater => next_x += 1,
                _ => {}
            }
        } else if dy > 0 {
            next_y += 1;
        } else if dy < 0 {
            next_y = next_y.saturating_sub(1);
        }
        EntityPosition::new(next_x, next_y)
    }
}

impl AiBehavior for BasicMonsterAI {
    fn clone_box(&self) -> Box<dyn AiBehavior> {
        Box::new(self.clone())
    }
    fn decide_next_action(
        &mut self,
        current_pos: &EntityPosition,
        game_state: &GameState,
    ) -> AiAction {
        let player_pos = &game_state.player.position;

        self.target_visible = self.is_player_in_fov(current_pos, player_pos);

        if self.target_visible {
            self.state = AiState::Chasing;
            self.last_known_player_pos = Some(player_pos.clone());
        } else {
            // If player was just visible, maybe keep chasing for a bit? Or switch to Idle.
            // For now, switch back to Idle immediately if not visible.
            self.state = AiState::Idle;
        }

        match self.state {
            AiState::Idle => {
                // Simple random movement: 25% chance to move NSEW, 75% chance to wait
                let mut rng = rand::rng();
                match rng.random_range(0..5) {
                    0 => AiAction::MoveTo(EntityPosition::new(
                        current_pos.x.saturating_sub(1),
                        current_pos.y,
                    )), // Left
                    1 => AiAction::MoveTo(EntityPosition::new(current_pos.x + 1, current_pos.y)), // Right
                    2 => AiAction::MoveTo(EntityPosition::new(
                        current_pos.x,
                        current_pos.y.saturating_sub(1),
                    )), // Up
                    3 => AiAction::MoveTo(EntityPosition::new(current_pos.x, current_pos.y + 1)), // Down
                    _ => AiAction::Wait, // Stay put
                }
            }
            AiState::Chasing => {
                if let Some(target_pos) = &self.last_known_player_pos {
                    // Basic: Move directly towards the player if adjacent, otherwise step towards
                    let dx = (current_pos.x as i32 - target_pos.x as i32).abs();
                    let dy = (current_pos.y as i32 - target_pos.y as i32).abs();

                    if dx <= 1 && dy <= 1 {
                        // Close enough to attack (placeholder)
                        AiAction::Attack(0) // Assuming player ID is 0
                    } else {
                        // Move towards the player
                        let next_pos = self.move_towards(current_pos, target_pos);
                        AiAction::MoveTo(next_pos)
                    }
                } else {
                    AiAction::Wait // Should not happen if Chasing state is managed correctly
                }
            }
        }
    }
}
