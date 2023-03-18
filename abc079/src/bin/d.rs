use std::{cmp::Ordering, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [[usize; 10]; 10],
        a: [[isize; w]; h],
    };

    let mut graph = vec![vec![]; 10];
    for i in 0..10 {
        for j in 0..10 {
            graph[i].push(Edge {
                node: j,
                cost: c[i][j],
            });
        }
    }

    let mut vec = vec![0; 10];
    for i in 0..h {
        for j in 0..w {
            if a[i][j] != -1 {
                vec[a[i][j] as usize] += 1;
            }
        }
    }

    let mut ans = 0;

    for i in 0..10 {
        let dist = dijkstra(&graph, i);
        ans += dist[1] * vec[i];
    }

    println!("{}", ans);
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
