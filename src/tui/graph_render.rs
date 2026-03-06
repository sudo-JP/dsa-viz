use crate::tui::App;
use ratatui::{layout::{Constraint, Layout, Rect, Spacing}, 
    style::{Color, Style}, widgets::{Block, Borders, Padding}, Frame};
use crate::{event::PathfindingEvent};

use Color::Gray as UnvisitColor;
use Color::Cyan as VisitingColor;
use Color::LightGreen as VisitedColor;
use Color::LightYellow as FoundColor;

impl App {

    pub fn render_graph(&self, frame: &mut Frame, area: Rect) {
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

    pub fn render_grid(&self, frame: &mut Frame, area: Rect) {
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
    pub fn step(&mut self) {
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
}
