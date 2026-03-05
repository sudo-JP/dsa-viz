use ratatui::{crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, layout::{Constraint, Layout, Rect, Spacing}, style::{Color, Style, Stylize}, widgets::{Block, BorderType, Borders, Paragraph}, DefaultTerminal, Frame};
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
            graph: Graph::new(2, 2),
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
        //self.render_title(frame, frame.area());
        self.render_grid(frame, frame.area());
    }

    fn render_title(&self, frame: &mut Frame, area: Rect)  {
        let block = Block::default()
                .title("Algorithm")
                .border_style(Style::default().fg(ratatui::style::Color::Magenta))
                .border_type(ratatui::widgets::BorderType::Rounded)
                .borders(Borders::ALL);
        let p = Paragraph::new("DFS")
            .block(block)
            .centered();
        frame.render_widget(p, area);
    }

    fn render_grid(&self, frame: &mut Frame, area: Rect) {
        let row_constraints = vec![Constraint::Fill(1); self.graph.cells.len()];
        let col_constraints = vec![Constraint::Fill(1); self.graph.cells[0].len()];

        let row_areas = Layout::vertical(&row_constraints)
            .spacing(Spacing::Overlap(1))
            .split(area);

        for (row_idx, row_area) in row_areas.iter().enumerate() {
            let col_areas = Layout::horizontal(&col_constraints)
                .spacing(Spacing::Overlap(1))
                .split(*row_area);

            for (col_idx, cell_area) in col_areas.iter().enumerate() {
                let color = if (row_idx + col_idx) % 2 == 0 {
                    Color::Rgb(235, 235, 235)
                } else {
                    Color::Rgb(119, 149, 86)
                };
let buf = frame.buffer_mut();

for y in cell_area.top()..cell_area.bottom() {
    for x in cell_area.left()..cell_area.right() {
        buf.get_mut(x, y).set_bg(color);
    }
}
            }
        }
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

