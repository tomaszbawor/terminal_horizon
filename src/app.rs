use crate::errors::AppError;
use crate::game::action_log::ActionType;
use crate::game::ai::AiAction;
use crate::game::entities::{Enemy, EntityPosition};
use crate::game::player::Player;
use crate::game::state::GameState;
use crate::game::{action_log::ActionLog, map::GameMap};
use crossterm::event::{self, Event};
use rand::{Rng, rng};

pub enum AppScreen {
    MainMenu,
    Options,
    Game,
}

pub struct App {
    pub screen: AppScreen,
    pub should_quit: bool,
    pub menu_index: usize,
    pub menu_items: Vec<String>,
    pub game_state: GameState,
}

const ENEMIES_COUNT: usize = 10;
const MAP_WIDTH: usize = 150;
const MAP_HEIGHT: usize = 120;

impl App {
    pub fn new() -> Self {
        let map = GameMap::new(MAP_WIDTH, MAP_HEIGHT);
        let mut enemies = vec![];

        let mut rand = rng();

        while enemies.len() < ENEMIES_COUNT {
            let x_pos = rand.random_range(1..MAP_WIDTH);
            let y_pos = rand.random_range(1..MAP_HEIGHT);

            if !map.is_wall(x_pos, y_pos) {
                enemies.push(Enemy::new(
                    EntityPosition::new(x_pos, y_pos),
                    "Goblin",
                    "g",
                    20,
                    5,
                    2,
                    8,
                ));
            }
        }

        Self {
            screen: AppScreen::MainMenu,
            should_quit: false,
            menu_index: 0,
            menu_items: vec![
                "New Game".to_string(),
                "Continue".to_string(),
                "Options".to_string(),
                "Quit".to_string(),
            ],
            game_state: GameState {
                player: Player::new("Hero", 100, 10, 5),
                map,
                enemies,
                journal: Vec::new(),
                turn: 0,
            },
        }
    }
    pub fn handle_events(&mut self) -> Result<bool, AppError> {
        // Using hypothetical AppError
        if let Event::Key(key) = event::read().map_err(AppError::Io)? {
            match self.screen {
                AppScreen::MainMenu => {
                    if let Some(action) = crate::input::handlers::handle_menu_input(key) {
                        self.apply_menu_action(action); // New method needed
                    }
                }
                AppScreen::Game => {
                    if let Some(action) = crate::input::handlers::handle_game_input(key) {
                        self.apply_game_action(action); // New method needed
                    }
                }
                AppScreen::Options => { /* Handle options input or keep todo!() */ }
            }
        }
        Ok(self.should_quit)
    }

    fn apply_menu_action(&mut self, action: crate::input::handlers::MenuAction) {
        use crate::input::handlers::MenuAction;
        match action {
            MenuAction::NavigateUp => {
                if self.menu_index > 0 {
                    self.menu_index -= 1;
                }
            }
            MenuAction::NavigateDown => {
                if self.menu_index < self.menu_items.len() - 1 {
                    self.menu_index += 1;
                }
            }
            MenuAction::Select => match self.menu_index {
                0 | 1 => self.screen = AppScreen::Game, // New Game or Continue
                2 => {}                                 // Options
                3 => self.should_quit = true,           // Quit
                _ => {}
            },
            MenuAction::Quit => self.should_quit = true,
        }
    }

    fn apply_game_action(&mut self, action: crate::input::handlers::GameAction) {
        use crate::input::handlers::{Direction, GameAction};
        let mut moved = false; // Track if player moved to update turn/log
        let mut player_took_action = false; // turn may be passed not only by moving

        let mut new_pos = (
            self.game_state.player.position.x,
            self.game_state.player.position.y,
        );

        match action {
            GameAction::OpenMenu => self.screen = AppScreen::MainMenu,
            GameAction::Quit => self.should_quit = true,
            GameAction::MovePlayer(dir) => {
                match dir {
                    Direction::Up => moved = self.game_state.player.move_up(&self.game_state.map),
                    Direction::Down => {
                        moved = self.game_state.player.move_down(&self.game_state.map)
                    }
                    Direction::Left => {
                        moved = self.game_state.player.move_left(&self.game_state.map)
                    }
                    Direction::Right => {
                        moved = self.game_state.player.move_right(&self.game_state.map)
                    }
                }
                if moved {
                    moved = true;
                    player_took_action = true;
                    self.game_state.journal.push(ActionLog::new(
                        self.game_state.turn,
                        ActionType::Movement {
                            x: new_pos.0,
                            y: new_pos.1,
                        },
                    ));
                }
            }
        }

        if player_took_action {
            self.game_state.turn += 1; // Increment turn only once after all actions resolve
            //
            // Store intended actions: (enemy_index, decided_action)
            let mut enemy_actions: Vec<(usize, AiAction)> =
                Vec::with_capacity(self.game_state.enemies.len());

            // Decide Actions
            for i in 0..self.game_state.enemies.len() {
                let enemy_pos = self.game_state.enemies[i].position.clone(); // Clone position for decision
                //
                let gs = self.game_state.clone();

                let ai_decision = self.game_state.enemies[i]
                    .ai_behavior
                    .decide_next_action(&enemy_pos, &gs); // Pass immutable game_state

                enemy_actions.push((i, ai_decision)); // Store decision
            }

            // Execute Actions
            for (enemy_index, action) in enemy_actions {
                match action {
                    AiAction::Wait => {
                        // Log enemy waiting (optional)
                    }
                    AiAction::MoveTo(next_pos) => {
                        // Check bounds and walls BEFORE updating position
                        if next_pos.x < self.game_state.map.width
                            && next_pos.y < self.game_state.map.height
                            && !self.game_state.map.is_wall(next_pos.x, next_pos.y)
                        {
                            // Check for collision with player (basic)
                            if next_pos != self.game_state.player.position {
                                // Check for collision with other enemies (basic)
                                let collision = self.game_state.enemies.iter().enumerate().any(
                                    |(idx, other)| idx != enemy_index && other.position == next_pos,
                                );

                                if !collision {
                                    self.game_state.enemies[enemy_index].position = next_pos;
                                    // Log enemy movement (optional)
                                }
                            } else {
                                // Enemy bumps into player - attack instead? Or just block.
                                // For now, block. Combat system needed.
                            }
                        }
                    }
                    AiAction::Attack(target_id) => {
                        // Implement combat logic here
                        // For now, just log (placeholder)
                        println!(
                            "Enemy {} attacks target {}!",
                            self.game_state.enemies[enemy_index].name, target_id
                        );
                        self.game_state.journal.push(ActionLog::new(
                            self.game_state.turn,
                            ActionType::MonsterAttack {
                                // You'll need to define this variant
                                attacker_name: self.game_state.enemies[enemy_index].name.clone(), // Immutable borrow needed here
                                target_name: self.game_state.player.name.clone(), // Immutable borrow needed here
                                damage: 0, // Placeholder damage
                            },
                        ));
                        // TODO: Apply damage to the player (would need mutable player borrow)
                    }
                }
            }
        }
    }
}
