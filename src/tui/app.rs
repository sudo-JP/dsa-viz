use ratatui::{crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, style::Style, widgets::{Block, Borders, Widget}, DefaultTerminal, Frame};
use std::io::Result;

use crate::{Graph};

pub struct App {
    exit: bool, 

    graph: Graph,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            graph: Graph::new(10, 10),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let block = Block::default()
            .title("Hello")
            .border_style(Style::default().fg(ratatui::style::Color::Magenta))
            .border_type(ratatui::widgets::BorderType::Rounded)
            .borders(Borders::ALL);
        frame.render_widget(block, frame.area()); 
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            },
            _ => {}
        };

        Ok(())
    }

    fn exit(&mut self) { self.exit = true; }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(), 
            _ => {}
        }
    }
}

