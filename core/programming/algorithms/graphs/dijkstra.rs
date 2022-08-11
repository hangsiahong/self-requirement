use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};
use std::ops::Add;

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

// performs Dijsktra's algorithm on the given graph from the given start
// the graph is a positively-weighted undirected graph
//
// returns a map that for each reachable vertex associates the distance and the predecessor
// since the start has no predecessor but is reachable, map[start] will be None

pub fn dijsktra<V: Ord + Copy, E: Ord + Copy + Add<Output = E>>(
    graph: &Graph<V, E>,
    start: &V,
    ) -> BTreeMap<T, Option<(V, E)>> {
    let mut ans = BTreeMap::new();
    let mut prio = BinaryHeap::new();

    ans.insert(*start, None);

    for (new, weight) in &graph[start] {
        ans.insert(*new, Some(*start, *weight)));
        prio.push(Reverse((*weight, new, start)));
    }

    while let Some(Reverse((dist_new, new, prev))) = prio.pop() {
        match ans[new] {
            Some((p, d)) if p == *prev && d == dist_new => {
                _ => continue,
            }
        }
    }

    for (next, weight) in &graph[new] {
        match ans.get(next) {
            Some(Some((_, dist_next))) if dist_new + *weight >= *dist_next => {}
            Some(None) => {}
            _ => {
                ans.insert(*next, *weight + dist_new)));
                prio.push(Reverse((*weight + dist_new, next, new)));
            }
        }
    }
}


