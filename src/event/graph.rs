use ratatui::style::Color;


#[derive(PartialEq, Clone, Copy, Eq, Hash, Debug)]
pub struct CellIndex {
    pub row: usize, 
    pub col: usize,
}

#[derive(Debug)]
pub enum PathfindingEvent {
    Visiting(CellIndex),
    Visited(CellIndex),
    Unvisit(CellIndex),
    Found(CellIndex),
}

