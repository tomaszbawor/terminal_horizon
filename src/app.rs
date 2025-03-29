use crate::game::action_log::ActionType;
use crate::game::player::Player;
use crate::game::state::GameState;
use crate::game::{action_log::ActionLog, map::GameMap};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::io;

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
                map: GameMap::new(50, 30),
                journal: Vec::new(),
                turn: 0,
            },
        }
    }

    pub fn handle_events(&mut self) -> io::Result<bool> {
        if let Event::Key(key) = event::read()? {
            match self.screen {
                AppScreen::MainMenu => self.handle_menu_input(key),
                AppScreen::Game => self.handle_game_input(key),
                AppScreen::Options => todo!(),
            }
        }
        Ok(self.should_quit)
    }

    fn handle_menu_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up => {
                if self.menu_index > 0 {
                    self.menu_index -= 1;
                }
            }
            KeyCode::Down => {
                if self.menu_index < self.menu_items.len() - 1 {
                    self.menu_index += 1;
                }
            }
            KeyCode::Enter => {
                match self.menu_index {
                    0 => self.screen = AppScreen::Game, // New Game
                    1 => self.screen = AppScreen::Game, // Continue
                    2 => {}                             // Options (not implemented yet)
                    3 => self.should_quit = true,       // Quit
                    _ => {}
                }
            }
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    fn handle_game_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.screen = AppScreen::MainMenu;
            }
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            KeyCode::Up | KeyCode::Char('w') => {
                self.game_state.player.move_up(&self.game_state.map);
                self.game_state.turn += 1;
            }
            KeyCode::Down | KeyCode::Char('s') => {
                self.game_state.player.move_down(&self.game_state.map);
                self.game_state.turn += 1;
            }
            KeyCode::Left | KeyCode::Char('a') => {
                self.game_state.player.move_left(&self.game_state.map);
                self.game_state.turn += 1;
            }
            KeyCode::Right | KeyCode::Char('d') => {
                self.game_state.player.move_right(&self.game_state.map);
                self.game_state.turn += 1;
            }
            _ => {}
        }

        self.game_state.journal.push(ActionLog::new(
            self.game_state.turn,
            ActionType::Movement {
                x: self.game_state.player.x,
                y: self.game_state.player.y,
            },
        ));
    }
}
