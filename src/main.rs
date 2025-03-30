#![allow(dead_code)]

mod app;
mod errors;
mod game;
mod input;
mod ui;

use app::{App, AppScreen};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use errors::AppError;
use input::events::EventHandler;
use ratatui::{Terminal, prelude::*};
use std::{error::Error, io};
use ui::ui;

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();
    let event_handler = EventHandler::new(250);

    // Main loop
    let res = run_app(&mut terminal, &mut app, event_handler);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Handle potential errors
    if let Err(err) = res {
        eprintln!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    mut event_handler: EventHandler,
) -> Result<(), AppError> {
    loop {
        // Draw UI
        terminal.draw(|f| ui(f, app))?;

        if event_handler
            .next()
            .map_err(|e| AppError::EventError(e.to_string()))?
        {
            match app.handle_events() {
                // handle_events now returns Result<(), AppError>
                Ok(should_quit) => {
                    if should_quit {
                        return Ok(());
                    }
                }
                Err(err) => {
                    // Log error maybe, then return it
                    eprintln!("Error during event handling: {}", err);
                    return Err(err); // Propagate the specific error
                }
            }
        }

        if matches!(app.screen, AppScreen::Game) {
            app.run_schedule();
        }
    }
}
