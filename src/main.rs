#![allow(dead_code)]

mod app;
mod game;
mod input;
mod ui;

use app::App;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
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
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    mut event_handler: EventHandler,
) -> io::Result<()> {
    loop {
        // Draw UI
        terminal.draw(|f| ui(f, app))?;

        // Handle events
        if event_handler.next()? {
            match app.handle_events() {
                Ok(should_quit) => {
                    if should_quit {
                        return Ok(());
                    }
                }
                Err(err) => {
                    return Err(io::Error::new(io::ErrorKind::Other, format!("{:?}", err)));
                }
            }
        }
    }
}
