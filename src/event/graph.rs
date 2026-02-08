pub type NodeID = i32;

pub enum GraphTraversalEvent {
    Visiting(NodeID),
    Visited(NodeID),
    Unvisit(NodeID),
}

