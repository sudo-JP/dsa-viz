//use rand::distr::Iter;
use std::collections::{HashSet, VecDeque};

use crate::{ds::graph::Graph, event::GraphTraversalEvent};

impl Graph {
    pub fn dfs(mut self) -> Vec<GraphTraversalEvent> {
        let start_idx = (rand::random::<i32>() as usize % self.adj_list.len()) as i32;
        let mut stack: Vec<i32> = vec![];
        let mut visited: HashSet<i32> = HashSet::new();

        stack.push(start_idx);
        visited.insert(start_idx);
        self.events.push(GraphTraversalEvent::Visiting(start_idx));

        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            self.events.push(GraphTraversalEvent::Visited(node));

            for neighbor in &self.adj_list[node as usize] {
                if !visited.contains(neighbor) {
                    stack.push(*neighbor);
                    visited.insert(*neighbor);
                    self.events.push(GraphTraversalEvent::Visiting(*neighbor));
                }
            }
        }

        self.events
    }

    pub fn bfs(mut self) -> Vec<GraphTraversalEvent> {
        let start_idx = (rand::random::<i32>() as usize % self.adj_list.len()) as i32;
        let mut queue: VecDeque<i32> = VecDeque::new();
        let mut visited: HashSet<i32> = HashSet::new();

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
    }
}
