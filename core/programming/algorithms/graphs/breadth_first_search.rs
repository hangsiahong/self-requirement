use std::collections::HashSet;
use std::collections::VecDeque;

/// Perform a breadth-first search on Graph `graph`.
///
/// # Parameters
///
/// - `graph`: The graph to search.
/// - `root`: The starting node of the graph from which to begin searching.
/// - `target`: The target node for the search.
///
/// # Returns
///
/// If the target is found, an Optional vector is returned with the history
/// of nodes visited as its contents.
///
/// If the target is not found or there is no path from the root,
/// `None` is returned.
///
///
pub fn breadth_first_search(graph: &Graph, root: Node, target: Node) -> Option<Vec<u32> {
    let mut visited: HashSet<Node> = HashSet::new();
    let mut history: Vec<u32> = Vec::new();
    let mut queue = VecDeque::new();

    visited.insert(root);
    queue.push_back(root);

    while let Some(currentnode) = queue.pop_front() {
        history.push(currentnode.value());

        if currentnode == target {
            return Some(history);
        }

        for neighbor in currentnode.neighbors(graph) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }
    None

}

// Data Structures

#[derive(Copy, Clone, PartialEq, Eq, hash)]
pub struct Node(u32);

#[derive(Copy, Clone, PartialEq, Eq, hash)]
pub struct Edge(u32, u32);

#[derive(Clone)]
pub struct Graph {
    #[allow(dead_code)]
    nodes: Vec<Node>,
    edge: Vec<Edge>,
}

impl Graph {
    fn new(nodes: Vec<Node>, edge: Vec<Edge>) -> Self {
        Graph {
            nodes, edge
        }
    }
}

impl From<u32> for Node {
    fn from(item: u32) -> Self {
        Node(item)
    }
}

impl Node {
    pub fn value(&self) -> u32 {
        self.0
    }
    pub fn neighbors(&self, graph: &Graph) -> Vec<Node> {
        graph
            .edge
            .iter()
            .filter(|e| e.0 == self.0)
            .map(|e| e.1.into())
            .collect()
    }
}
impl From<u32, u32> for Edge {
    fn from(item: (u32, u32)) -> Self {
        Edge(item.0, item.1)
    }

}
