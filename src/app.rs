use std::ops::Range;

use crate::errors::AppError;
use crate::game::action_log::ActionType;
use crate::game::entities::Enemy;
use crate::game::player::Player;
use crate::game::state::GameState;
use crate::game::{action_log::ActionLog, map::GameMap};
use crossterm::event::{self, Event};
use rand::{Rng, random_range, rng};

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

impl App {
    pub fn new() -> Self {
        let map = GameMap::new(150, 130);
        let enemies = vec![Enemy::new(20, 20, "Goblin", "g", 20, 5, 2, 8)];

        //let rand = rng();

        //while enemies.len() < 4 {
        // generate random cooridinates for enemies
        //enemies.push(Enemy::new(20, 20, "Goblin", "g", 20, 5, 2, 8));
        //   break;
        //}

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
        let mut new_pos = (self.game_state.player.x, self.game_state.player.y);

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
                    new_pos = (self.game_state.player.x, self.game_state.player.y);
                    //TODO: Move Enemy AI
                }
            }
        }

        if moved {
            self.game_state.turn += 1;
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
