use crate::tui::App;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::{io::Result, time::Duration};

impl App {
    // KEY INPUT EVENT 
    pub fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(0))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                },
                _ => {}
            };

        }

        Ok(())
    }

    // EXIT 
    pub fn exit(&mut self) { self.exit = true; }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(), 
            _ => {}
        }
    }
}
