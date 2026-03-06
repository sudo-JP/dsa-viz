use ratatui::{layout::{Constraint, Layout, Rect}, 
    style::{Color, Style}, widgets::{Block, Borders, Paragraph}, DefaultTerminal, Frame};
use std::{io::Result};

use crate::{event::PathfindingEvent, Graph};

pub struct App {
    pub exit: bool, 
    pub graph: Graph,
    pub events: Vec<PathfindingEvent>, 
    pub curr_event_idx: usize, 
    pub cell_colors: Vec<Vec<Color>>,
}

use Color::Gray as UnvisitColor;

impl App {
    pub fn new(graph: Graph, events: Vec<PathfindingEvent>) -> Self {
        let cell_colors = vec![vec![UnvisitColor; graph.cells[0].len()]; graph.cells.len()];

        Self {
            exit: false,
            graph, 
            events: events,
            curr_event_idx: 0, 
            cell_colors
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.step();
            self.handle_events()?;
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let div = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(0),
        ]).split(frame.area());

        self.render_title(frame, div[0]);
        self.render_graph(frame, div[1]);
    }

    fn render_title(&self, frame: &mut Frame, area: Rect)  {
        let block = Block::default()
                .title("dsa-viz")
                .border_style(Style::default().fg(ratatui::style::Color::Magenta))
                .border_type(ratatui::widgets::BorderType::Rounded)
                .borders(Borders::ALL);
        let p = Paragraph::new("DFS")
            .block(block)
            .centered();
        frame.render_widget(p, area);
    }


}

