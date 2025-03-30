use crossterm::event::{KeyCode, KeyEvent};

// Represents actions possible from the menu
pub enum MenuAction {
    NavigateUp,
    NavigateDown,
    Select,
    Quit,
}

// Represents actions possible in the game
#[derive(Clone, PartialEq, PartialOrd)]
pub enum GameAction {
    MovePlayer(Direction), // Define Direction enum (Up, Down, Left, Right)
    OpenInventory,
    OpenMenu,
    Quit,
}

#[derive(Clone, PartialEq, PartialOrd)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn handle_menu_input(key: KeyEvent) -> Option<MenuAction> {
    match key.code {
        KeyCode::Up => Some(MenuAction::NavigateUp),
        KeyCode::Down => Some(MenuAction::NavigateDown),
        KeyCode::Enter => Some(MenuAction::Select),
        KeyCode::Char('q') => Some(MenuAction::Quit),
        _ => None,
    }
}

pub fn handle_game_input(key: KeyEvent) -> Option<GameAction> {
    match key.code {
        KeyCode::Esc => Some(GameAction::OpenMenu),
        KeyCode::Char('q') => Some(GameAction::Quit),
        KeyCode::Up | KeyCode::Char('w') => Some(GameAction::MovePlayer(Direction::Up)),
        KeyCode::Down | KeyCode::Char('s') => Some(GameAction::MovePlayer(Direction::Down)),
        KeyCode::Left | KeyCode::Char('a') => Some(GameAction::MovePlayer(Direction::Left)),
        KeyCode::Right | KeyCode::Char('d') => Some(GameAction::MovePlayer(Direction::Right)),
        _ => None,
    }
}
