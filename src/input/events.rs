use crossterm::event::{self, Event};
use std::{io, time::{Duration, Instant}};

pub struct EventHandler {
    pub tick_rate: Duration,
    pub last_tick: Instant,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        Self {
            tick_rate: Duration::from_millis(tick_rate),
            last_tick: Instant::now(),
        }
    }

    pub fn next(&mut self) -> io::Result<bool> {
        if event::poll(Duration::from_millis(50))? {
            return Ok(true);
        }
        
        if self.last_tick.elapsed() >= self.tick_rate {
            self.last_tick = Instant::now();
        }
        
        Ok(false)
    }
}
