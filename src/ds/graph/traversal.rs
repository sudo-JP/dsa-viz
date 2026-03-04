//use rand::distr::Iter;
use std::collections::{HashMap};

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

        let mut path: Vec<CellIndex> = vec![];
        let mut current = self.target_idx;
        while let Some(&parent) = backtrack.get(&current) {
            path.push(current);
            current = parent;
        }
        path.push(current);
        path.reverse();
        events.push(PathfindingEvent::PathFound(path));
        
        events
    }

    /*pub fn bfs(mut self) -> Vec<Vec<PathfindingEvent>> {
        let start_idx = rand::random::<i32>() as usize % self.adj_list.len();
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut visited: HashSet<usize> = HashSet::new();

        queue.push_back(start_idx);
        visited.insert(start_idx);
        self.events.push(GraphTraversalEvent::Visiting(start_idx));

        while queue.len() > 0 {
            let node = match queue.pop_front() {
                Some(p) => p,
                None => panic!(),
            };
            self.events.push(GraphTraversalEvent::Visited(node));

            for neighbor in &self.adj_list[node as usize] {
                if !visited.contains(&neighbor) {
                    //println!("Visiting soon {neightbors}");
                    queue.push_back(*neighbor);
                    visited.insert(*neighbor);
                    self.events.push(GraphTraversalEvent::Visiting(*neighbor));
                }
            }
        }

        self.events
    }*/

}
