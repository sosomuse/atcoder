use std::collections::{BTreeSet, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    };

    let mut graph = vec![vec![]; n];
    let mut new_uv = switch_order(&uv);
    new_uv.sort();

    for (u, v) in uv {
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
    }

    let new_uv = nonexistes_edges(&graph);
    let mut ans = 0;

    for (u, v) in new_uv {
        let mut c = graph.clone();
        let mut colors = vec![0; n];
        c[u].push(v);

        if is_bipartite(0, &c, &mut colors, 1) {
            ans += 1;
        }
    }

    println!("{}", ans);
}

fn is_bipartite(v: usize, graph: &Vec<Vec<usize>>, colors: &mut Vec<isize>, color: isize) -> bool {
    let mut queue = VecDeque::new();
    queue.push_back((v, color));

    while let Some((v, color)) = queue.pop_front() {
        colors[v] = color;

        for &to in &graph[v] {
            if colors[to] == color {
                return false;
            }

            if colors[to] == 0 {
                queue.push_back((to, -color));
            }
        }
    }

    true
}

fn nonexistes_edges(graph: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut new_uv = vec![];
    let mut new_graph = vec![BTreeSet::new(); graph.len()];

    for i in 0..graph.len() {
        for &j in &graph[i] {
            new_graph[i].insert(j);
        }
    }

    for i in 0..graph.len() {
        for j in i + 1..graph.len() {
            if !new_graph[i].contains(&j) {
                new_uv.push((i, j));
            }
        }
    }

    new_uv
}

fn switch_order(uv: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut new_uv = vec![];

    for (u, v) in uv {
        match u.cmp(v) {
            std::cmp::Ordering::Less => new_uv.push((*u, *v)),
            std::cmp::Ordering::Greater => new_uv.push((*v, *u)),
            std::cmp::Ordering::Equal => (),
        }
    }

    new_uv
}
