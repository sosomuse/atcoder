use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        uvw: [(Usize1, Usize1, usize); n - 1],
    };

    let mut graph = vec![vec![]; n];
    let mut colors: Vec<isize> = vec![-1; n];

    for (u, v, w) in uvw {
        graph[u].push(Edge { node: v, cost: w });
        graph[v].push(Edge { node: u, cost: w });
    }

    bipartite(0, &graph, &mut colors, 100000000, 0);

    for i in 0..n {
        if colors[i] == 0 {
            println!("0");
        } else {
            println!("1");
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    node: usize,
    cost: usize,
}

fn bipartite(v: usize, graph: &Vec<Vec<Edge>>, colors: &mut Vec<isize>, p: usize, color: isize) {
    colors[v] = color;

    for &next in &graph[v] {
        if next.node == p {
            continue;
        }

        if next.cost % 2 == 0 {
            bipartite(next.node, graph, colors, v, color);
        } else {
            bipartite(next.node, graph, colors, v, 1 - color);
        }
    }
}
