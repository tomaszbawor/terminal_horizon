pub mod app_log;
pub mod game_area;
pub mod game_ui;
pub mod main_menu;
pub mod map;
pub mod sidebar;

use crate::app::{App, AppScreen};
use ratatui::prelude::*;

pub fn ui(f: &mut Frame, app: &App) {
    match app.screen {
        AppScreen::MainMenu => main_menu::render(f, app),
        AppScreen::Game => game_ui::render(f, app),
        AppScreen::Options => todo!(),
    }
}
