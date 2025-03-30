use crate::app::GameInputAction; // Need App temporarily for input action
use crate::game::components::*;
use crate::game::map::GameMap;
use crate::input::handlers::{Direction, GameAction};
use bevy_ecs::prelude::*;

use super::state::GameTurn;

// System to process player input and add intent components
pub fn player_input_system(
    mut commands: Commands, // Use Commands to add/remove components/entities
    game_input_action: ResMut<GameInputAction>, // Querying App is generally discouraged, better ways exist
    player_query: Query<Entity, With<Player>>,  // Find the player entity
) {
    let app = game_input_action; // Assumes only one App entity (or manage state differently)
    let player_entity = player_query.get_single();

    if let (Ok(player_ent), Some(action)) = (player_entity, app.0.clone()) {
        match action {
            GameAction::MovePlayer(direction) => {
                // Add a "WantsToMove" component to the player entity
                commands
                    .entity(player_ent)
                    .insert(WantsToMove { direction });
            }
            GameAction::OpenInventory => { /* Add WantsToOpenInventory component? */ }
            // OpenMenu and Quit are handled in app.rs for now
            _ => {}
        }
    }
    // The game_input_action is consumed here or in app.run_schedule
}

// System to handle movement intents
pub fn movement_system(
    mut commands: Commands,
    mut movers: Query<(Entity, &mut Position, &WantsToMove)>,
    //blockers: Query<&Position, With<BlocksTile>>, // Query for positions of blocking entities
    map: Res<GameMap>, // Access the map resource
) {
    for (entity, mut pos, intent) in movers.iter_mut() {
        let current_x = pos.x;
        let current_y = pos.y;

        let (next_x, next_y) = match intent.direction {
            Direction::Up => (current_x, current_y.saturating_sub(1)),
            Direction::Down => (current_x, current_y.saturating_add(1)),
            Direction::Left => (current_x.saturating_sub(1), current_y),
            Direction::Right => (current_x.saturating_add(1), current_y),
        };

        // Check map bounds and walls
        if next_x < map.width && next_y < map.height && !map.is_wall(next_x, next_y) {
            // Check collision with blocking entities
            // let collision = blockers
            //     .iter()
            //     .any(|blocker_pos| blocker_pos.x == next_x && blocker_pos.y == next_y);
            let collision = false;

            if !collision {
                pos.x = next_x;
                pos.y = next_y;
                // Add log entry here or via a dedicated logging system
            }
        }

        // Remove the intent component after processing
        commands.entity(entity).remove::<WantsToMove>();
    }
}

// Basic AI System Example
pub fn ai_system(
    mut commands: Commands,
    mut ai_query: Query<(Entity, &mut BasicAi, &Position), With<Enemy>>,
    player_query: Query<&Position, With<Player>>,
    // Optional: query for other entities if AI needs awareness
    // map: Res<GameMap>, // Needed for pathfinding checks potentially
    // mut log: ResMut<ActionJournal>,
) {
    let player_pos = match player_query.get_single() {
        Ok(p) => p,
        Err(_) => return, // No player found
    };

    for (entity, mut ai, position) in ai_query.iter_mut() {
        // Simplified decision logic from BasicMonsterAI::decide_next_action [cite: 32]
        let target_visible = {
            // FOV check
            let dx = (position.x as i32 - player_pos.x as i32).abs();
            let dy = (position.y as i32 - player_pos.y as i32).abs();
            dx as u32 <= ai.fov_radius && dy as u32 <= ai.fov_radius
        };

        if target_visible {
            ai.state = AiState::Chasing;
            ai.last_known_player_pos = Some(player_pos.clone());
        } else {
            ai.state = AiState::Idle; // Or more complex logic
        }

        let action = match ai.state {
            AiState::Idle => {
                // Random move/wait
                // ... (Implement random move logic similar to old AI)
                // For now, just wait
                None
            }
            AiState::Chasing => {
                if let Some(target_pos) = &ai.last_known_player_pos {
                    let dx = (position.x as i32 - target_pos.x as i32).abs();
                    let dy = (position.y as i32 - target_pos.y as i32).abs();

                    if dx <= 1 && dy <= 1 {
                        // Close enough to attack
                        // Find player entity (requires querying Entity With<Player>)
                        // Some(WantsToAttack { target: player_entity })
                        //
                        //TODO: Add handle attack and log entry to journal

                        None // Placeholder
                    } else {
                        // Move towards target
                        let dx_dir = target_pos.x as i32 - position.x as i32;
                        let dy_dir = target_pos.y as i32 - position.y as i32;
                        let direction = if dx_dir.abs() > dy_dir.abs() {
                            if dx_dir > 0 {
                                Direction::Right
                            } else {
                                Direction::Left
                            }
                        } else if dy_dir > 0 {
                            Direction::Down
                        } else {
                            Direction::Up
                        };
                        Some(WantsToMove { direction })
                    }
                } else {
                    None
                }
            }
        };

        // Add intent component based on action
        if let Some(move_intent) = action {
            commands.entity(entity).insert(move_intent);
        }
        // else if let Some(attack_intent) = action {
        //    commands.entity(entity).insert(attack_intent);
        // }
    }
}

// Example system to increment the turn counter
pub fn update_turn_system(mut turn: ResMut<GameTurn>) {
    turn.0 += 1;
}
