use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
    };

    let check_hex = |x1: isize, y1: isize, x2: isize, y2: isize| -> bool {
        if x1 - 1 == x2 && y1 - 1 == y2 {
            return true;
        }

        if x1 - 1 == x2 && y1 == y2 {
            return true;
        }

        if x1 == x2 && y1 - 1 == y2 {
            return true;
        }

        if x1 == x2 && y1 + 1 == y2 {
            return true;
        }

        if x1 + 1 == x2 && y1 == y2 {
            return true;
        }

        if x1 + 1 == x2 && y1 + 1 == y2 {
            return true;
        }

        return false;
    };

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            if check_hex(xy[i].0, xy[i].1, xy[j].0, xy[j].1) {
                graph[i].push(j);
            }
        }
    }

    let mut visited: Vec<bool> = vec![false; graph.len()];
    let mut count: usize = 0;

    for i in 0..n {
        if visited[i] {
            continue;
        }
        dfs_inner(i, &graph, &mut visited);
        count += 1;
    }

    println!("{}", count);
}

fn dfs_inner(v: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[v] = true;

    for &w in graph[v].iter() {
        if !visited[w] {
            dfs_inner(w, graph, visited);
        }
    }
}
