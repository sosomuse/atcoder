use proconio::{input, marker::Usize1};
use std::{
    cmp::{max, Ordering},
    collections::BinaryHeap,
};

fn main() {
    input! {
        n: usize,
        m: usize,
        h: [usize; n],
        uv: [(Usize1, Usize1); m],
    };

    let mut graph = vec![vec![]; n];

    for (u, v) in uv {
        graph[u].push(Edge {
            node: v,
            cost: h[v].saturating_sub(h[u]),
        });
        graph[v].push(Edge {
            node: u,
            cost: h[u].saturating_sub(h[v]),
        });
    }

    let mut ans = 0;
    dijkstra(&graph, 0, &mut ans, &h);

    println!("{}", ans);
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

fn dijkstra(graph: &Vec<Vec<Edge>>, start: usize, ans: &mut usize, h: &Vec<usize>) -> Vec<usize> {
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

        *ans = max(
            *ans,
            h[0].saturating_sub(h[position])
                .saturating_sub(dist[position]),
        );

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
