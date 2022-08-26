use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        m: usize,
        xy: [(usize, usize); m],
    };

    let mut ans: usize = 10000000000;
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];

    for i in 1..=n {
        for j in 1..=n {
            if i == j {
                continue;
            }
            graph[i].push(j);
        }
    }

    for i in 0..m {
        graph[xy[i].0].retain(|&y| y != xy[i].1);
        graph[xy[i].1].retain(|&y| y != xy[i].0);
    }

    dbg!(&graph);

    for i in 1..=n {
        let v = dfs(i, &graph);

        let mut tmp = 0;

        for j in 0..v.len() {
            tmp += a[v[j] - 1][j];
        }
        // dbg!(&v, &tmp);

        ans = ans.min(tmp);
    }

    println!("{}", ans);
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut visited: Vec<bool> = vec![false; graph.len()];
    let mut ans: Vec<usize> = vec![];
    dfs_inner(v, graph, &mut visited, &mut ans);
    ans
}

fn dfs_inner(v: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, ans: &mut Vec<usize>) {
    visited[v] = true;
    ans.push(v);

    for &w in graph[v].iter() {
        if !visited[w] {
            dfs_inner(w, graph, visited, ans);
        }
    }
}
