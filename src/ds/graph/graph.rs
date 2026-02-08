use crate::event::GraphTraversalEvent;

pub struct Graph {
    pub adj_list: Vec<Vec<i32>>,
    pub events: Vec<GraphTraversalEvent>,
}

impl Graph {
    pub fn new(n: i32) -> Self {
        let mut list = vec![];
        let node1 = vec![1, 3];
        let node2 = vec![0, 2];
        let node3 = vec![1, 3];
        let node4 = vec![0, 2];
        list.push(node1);
        list.push(node2);
        list.push(node3);
        list.push(node4);

        let mut events: Vec<GraphTraversalEvent> = vec![];
        events.push(GraphTraversalEvent::Unvisit(0));
        events.push(GraphTraversalEvent::Unvisit(1));
        events.push(GraphTraversalEvent::Unvisit(2));
        events.push(GraphTraversalEvent::Unvisit(3));

        Self {
            adj_list: list,
            events: events,
        }
    }
}


