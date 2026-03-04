use crate::event::{CellIndex, PathfindingEvent};

#[derive(PartialEq, Clone, Copy)]
pub enum Cell {
    Empty, 
    Wall, 
    Start, 
    Target,
}

pub struct Graph {
    pub cells: Vec<Vec<Cell>>,
    pub events: Vec<Vec<PathfindingEvent>>,
    pub start_idx: CellIndex,
    pub target_idx: CellIndex,
}

impl Graph {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells: Vec<Vec<Cell>> = vec![];
        let events: Vec<Vec<PathfindingEvent>> = vec![];

        for _ in 0..height {
            let mut row: Vec<Cell> = vec![];
            for _ in 0..width{
                row.push(Cell::Empty);
            } 
            cells.push(row);
        }

        // Hardcode for now, let starting at (0, 0)
        // Target at (height - 1, width - 1)
        cells[0][0] = Cell::Start;
        cells[height - 1][width - 1] = Cell::Target;

        Self {
            start_idx: CellIndex{row: 0, col: 0}, 
            target_idx: CellIndex{row: height - 1, col: width - 1},
            cells: cells,
            events: events,
        }
    }
}


