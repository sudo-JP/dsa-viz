//use rand::distr::Iter;
use std::collections::{HashMap, VecDeque};

use crate::{ds::graph::{graph::Cell, Graph}, event::{CellIndex, PathfindingEvent}};

impl Graph {
    fn is_in_bound(&self, i: i32, j: i32) -> Option<CellIndex> {
        if i < 0 || i >= self.cells.len() as i32 { None }
        else if j < 0 || j >= self.cells[0].len() as i32 { None }
        else { Some(CellIndex{row: i as usize, col: j as usize}) }
    }

    pub fn dfs(&self) -> Vec<PathfindingEvent> {
        let start = CellIndex{row: 0, col: 0};
        let mut stack = vec![start];
        let mut visited = vec![vec![false; self.cells[0].len()]; self.cells.len()];
        let mut events = vec![];
        let mut backtrack: HashMap<CellIndex, CellIndex> = HashMap::new();
        
        while let Some(cell) = stack.pop() {
            if cell == self.target_idx {
                events.push(PathfindingEvent::Visiting(cell));
                break;
            }
            if visited[cell.row][cell.col] { continue; }
            if self.cells[cell.row][cell.col] == Cell::Wall { continue; }
            
            events.push(PathfindingEvent::Visiting(cell));
            
            // check 4 neighbors, push valid ones
            let neighbors = [
                (1, 0),
                (0, 1),
                (-1, 0),
                (0, -1),
            ];
            for (i, j) in neighbors {
                // Check in bound
                if let Some(valid_neigh) = self.is_in_bound(cell.row as i32 + i, 
                    cell.col as i32 + j) 

                // Check unvisited
                && !visited[valid_neigh.row][valid_neigh.col] &&

                // Check if not wall
                self.cells[valid_neigh.row][valid_neigh.col] != Cell::Wall {
                    backtrack.insert(valid_neigh, cell);
                    stack.push(valid_neigh);
                }
            }
            visited[cell.row][cell.col] = true;
            events.push(PathfindingEvent::Visited(cell));
        }

        let mut current = self.target_idx;
        while let Some(&parent) = backtrack.get(&current) {
            events.push(PathfindingEvent::Found(current));
            current = parent;
        }
        events.push(PathfindingEvent::Found(current));
        
        events
    }


    pub fn bfs(&self) -> Vec<PathfindingEvent> {
        let start = CellIndex{row: 0, col: 0};
        let mut queue: VecDeque<CellIndex> = VecDeque::new();
        queue.push_front(start);
        let mut visited = vec![vec![false; self.cells[0].len()]; self.cells.len()];
        let mut events = vec![];
        let mut backtrack: HashMap<CellIndex, CellIndex> = HashMap::new();
        
        while let Some(cell) = queue.pop_back() {
            if cell == self.target_idx {
                events.push(PathfindingEvent::Visiting(cell));
                break;
            }
            if visited[cell.row][cell.col] { continue; }
            if self.cells[cell.row][cell.col] == Cell::Wall { continue; }
            
            events.push(PathfindingEvent::Visiting(cell));
            
            // check 4 neighbors, push valid ones
            let neighbors = [
                (1, 0),
                (0, 1),
                (-1, 0),
                (0, -1),
            ];
            for (i, j) in neighbors {
                // Check in bound
                if let Some(valid_neigh) = self.is_in_bound(cell.row as i32 + i, 
                    cell.col as i32 + j) 

                // Check unvisited
                && !visited[valid_neigh.row][valid_neigh.col] &&

                // Check if not wall
                self.cells[valid_neigh.row][valid_neigh.col] != Cell::Wall {
                    backtrack.insert(valid_neigh, cell);
                    queue.push_front(valid_neigh);
                }
            }
            visited[cell.row][cell.col] = true;
            events.push(PathfindingEvent::Visited(cell));
        }

        let mut current = self.target_idx;
        while let Some(&parent) = backtrack.get(&current) {
            events.push(PathfindingEvent::Found(current));
            current = parent;
        }
        events.push(PathfindingEvent::Found(current));
        
        events
    }
}
