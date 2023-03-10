use std::{cmp::Ordering, collections::BinaryHeap};

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        h: usize,
        w: usize,
        (ch, cw): (Usize1, Usize1),
        (dh, dw): (Usize1, Usize1),
        s: [Chars; h],
    };

    let graph = create_maze_graph(h, w, &s, '#');
    let dist = dijkstra(&graph, w * ch + cw);

    let ans = dist[w * dh + dw];

    if ans == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", dist[w * dh + dw])
    }
}

// ダイクストラ法
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

fn create_maze_graph(h: usize, w: usize, s: &Vec<Vec<char>>, wall: char) -> Vec<Vec<Edge>> {
    let mut graph: Vec<Vec<Edge>> = vec![vec![]; h * w];

    for i in 0..h {
        let v = &s[i];
        for j in 0..w {
            let ch = v[j];
            if ch == wall {
                continue;
            }

            let p = w * i + j;
            // up
            if i > 0 && s[i - 1][j] != wall {
                graph[p].push(Edge {
                    node: w * (i - 1) + j,
                    cost: 0,
                });
            }
            // down
            if i != (h - 1) && s[i + 1][j] != wall {
                graph[p].push(Edge {
                    node: w * (i + 1) + j,
                    cost: 0,
                });
            }
            // left
            if j > 0 && s[i][j - 1] != wall {
                graph[p].push(Edge {
                    node: p - 1,
                    cost: 0,
                });
            }
            // right
            if j != (w - 1) && s[i][j + 1] != wall {
                graph[p].push(Edge {
                    node: p + 1,
                    cost: 0,
                });
            }

            for x in -2isize..=2 {
                for y in -2isize..=2 {
                    if (i as isize) < -x {
                        continue;
                    }
                    if (j as isize) < -y {
                        continue;
                    }
                    if h as isize - 1 < i as isize + x {
                        continue;
                    }
                    if w as isize - 1 < j as isize + y {
                        continue;
                    }

                    if s[(i as isize + x) as usize][(j as isize + y) as usize] != wall {
                        graph[p].push(Edge {
                            node: w * (i as isize + x) as usize + (j as isize + y) as usize,
                            cost: 1,
                        })
                    }
                }
            }
        }
    }

    graph
}
