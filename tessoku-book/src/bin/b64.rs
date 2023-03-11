use std::{collections::BinaryHeap, vec};

use proconio::input;
use std::cmp::Ordering;

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

    let dist = dijkstra(&graph, 1);

    // Nまでの経路復元
    let mut ans = vec![];
    let mut current = n;
    ans.push(current);

    while current != 1 {
        for edge in &graph[current] {
            if dist[current] >= edge.cost && dist[current] - edge.cost == dist[edge.node] {
                current = edge.node;
                ans.push(current);
                break;
            }
        }
    }

    ans.reverse();

    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }
}

// ダイクストラ法
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
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

fn dijkstra(graph: &Vec<Vec<Edge>>, start: usize) -> Vec<usize> {
    let mut dist: Vec<_> = (0..graph.len()).map(|_| std::usize::MAX).collect();
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if cost > dist[position] {
            continue;
        }

        for edge in &graph[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    dist
}
