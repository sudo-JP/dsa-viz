use ratatui::{crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, layout::{Constraint, Layout, Rect, Spacing}, style::{Color, Style}, widgets::{Block, Borders, Padding, Paragraph}, DefaultTerminal, Frame};
use std::{io::Result, time::Duration};

use crate::{event::PathfindingEvent, Graph};

pub struct App {
    exit: bool, 
    graph: Graph,
    events: Vec<PathfindingEvent>, 
    curr_event_idx: usize, 
    cell_colors: Vec<Vec<Color>>,
}

use Color::Gray as UnvisitColor;
use Color::Cyan as VisitingColor;
use Color::LightGreen as VisitedColor;
use Color::LightYellow as FoundColor;

impl App {
    pub fn new() -> Self {
        let graph = Graph::new(10, 10);
        let events = graph.dfs();
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
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let div = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(0),
        ]).split(frame.area());

        self.render_title(frame, div[0]);
        self.render_dsa_viz(frame, div[1]);
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

    fn render_dsa_viz(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title("graph visualizer")
            .border_style(Style::default().fg(ratatui::style::Color::Magenta))
            .border_type(ratatui::widgets::BorderType::Rounded)
            .padding(Padding::uniform(1))
            .borders(Borders::ALL);
        
        let inner = block.inner(area);
        self.render_grid(frame, inner);
        frame.render_widget(block, area);
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
                let block = Block::bordered()
                    .merge_borders(ratatui::symbols::merge::MergeStrategy::Exact);

                let inner = block.inner(*cell_area);

                frame.render_widget(block, *cell_area);

                // fill interior
                let fill = Block::new().style(Style::default()
                    .bg(self.cell_colors[row_idx][col_idx]));
                frame.render_widget(fill, inner);
            }
        }
    }

    // RENDERING STEP 

    fn step(&mut self) {
        if let Some(event) = self.events.get(self.curr_event_idx) {
            match event {
            PathfindingEvent::Visiting(cell) => 
                self.cell_colors[cell.row][cell.col] = VisitingColor,
            PathfindingEvent::Visited(cell) => 
                self.cell_colors[cell.row][cell.col] = VisitedColor,
            PathfindingEvent::Unvisit(cell) => 
                self.cell_colors[cell.row][cell.col] = UnvisitColor,
            PathfindingEvent::Found(cell) => 
                self.cell_colors[cell.row][cell.col] = FoundColor,
            };
        }
        self.curr_event_idx += 1;
    }

    // KEY INPUT EVENT 
    fn handle_events(&mut self) -> Result<()> {
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

    fn exit(&mut self) { self.exit = true; }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(), 
            _ => {}
        }
    }
}

