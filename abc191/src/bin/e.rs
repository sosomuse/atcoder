use std::{cmp::Ordering, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, usize); m],
    };

    let mut graph = vec![vec![]; n];
    for (a, b, c) in abc {
        graph[a].push(Edge { node: b, cost: c });
    }

    for i in 0..n {
        let dist = dijkstra(&graph, i);
        if dist[i] == std::usize::MAX {
            println!("-1");
        } else {
            println!("{}", dist[i]);
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Edge {
    node: usize,
    cost: usize,
}

fn dijkstra(graph: &Vec<Vec<Edge>>, start: usize) -> Vec<usize> {
    let mut dist: Vec<_> = (0..graph.len()).map(|_| std::usize::MAX).collect();
    let mut heap = BinaryHeap::new();

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
