use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut t_list: Vec<usize> = vec![0; n + 1];
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];

    for i in 1..=n {
        input! {
            t: usize,
            k: usize,
            a: [usize; k],
        };

        t_list[i] = t;
        graph[i] = a;
    }

    let tmp = dfs(n, &graph);
    let mut ans = 0;

    // dbg!(&tmp);

    for v in tmp {
        ans += t_list[v];
    }

    println!("{:?}", ans);
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
