use dsa_viz::{ds::graph::Graph, event::GraphTraversalEvent};


fn main() {
    let g  = Graph::new(0);
    let v = g.dfs();
    for item in v {
        match item {
            GraphTraversalEvent::Unvisit(i) => { println!("unvisit {i}")},
            GraphTraversalEvent::Visiting(i) => { println!("visiting {i}")},
            GraphTraversalEvent::Visited(i) => { println!("visited {i}")},
        }
    }
}
