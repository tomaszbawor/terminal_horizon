pub mod main_menu;
pub mod game_ui;
pub mod sidebar;
pub mod map;

use crate::app::{App, AppScreen};
use ratatui::{prelude::*, widgets::*};

pub fn ui(f: &mut Frame, app: &App) {
    match app.screen {
        AppScreen::MainMenu => main_menu::render(f, app),
        AppScreen::Game => game_ui::render(f, app),
    }
}
