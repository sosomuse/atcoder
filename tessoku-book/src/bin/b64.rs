use std::{collections::BinaryHeap, vec};

use proconio::input;
use std::cmp::Ordering;

// ダイクストラ法（経路復元）

#[derive(Copy, Clone, Eq, PartialEq)]

struct State {
    cost: usize,
    position: usize,
    pre_node: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
struct Edge {
    node: usize,
    cost: usize,
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m],
    };

    let mut graph: Vec<Vec<Edge>> = vec![vec![]; n + 1];

    for (a, b, c) in abc {
        graph[a].push(Edge { node: b, cost: c });
        graph[b].push(Edge { node: a, cost: c });
    }

    if let Some((_, ans)) = shortest_path(&graph, 1, n) {
        for i in 0..ans.len() {
            print!("{} ", ans[i]);
        }
    }
}

fn shortest_path(graph: &Vec<Vec<Edge>>, start: usize, end: usize) -> Option<(usize, Vec<usize>)> {
    let mut dist: Vec<_> = (0..graph.len()).map(|_| std::usize::MAX).collect();
    let mut pre_nodes: Vec<usize> = (0..graph.len()).map(|i| i).collect();
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
        pre_node: 0,
    });

    while let Some(State {
        cost,
        position,
        pre_node,
    }) = heap.pop()
    {
        if cost > dist[position] {
            continue;
        }

        pre_nodes[position] = pre_node;

        if position == end {
            let mut v = end;
            let mut path = vec![end];
            while v != start {
                path.push(pre_nodes[v]);
                v = pre_nodes[v];
            }
            path.reverse();
            return Some((cost, path));
        }

        for edge in &graph[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
                pre_node: position,
            };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    None
}
