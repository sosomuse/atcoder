fn main() {
    let v = vec![vec![0, 1], vec![2], vec![1]];
    let mut visited = vec![false; v.len() + 1];
    let mut ans: Vec<usize> = vec![];
    dfs(1, &v, &mut visited, &mut ans);
    println!("{:?}", ans);

    let v = vec![vec![0, 1], vec![2], vec![1]];
    let mut visited = vec![false; v.len() + 1];
    let mut ans: Vec<usize> = vec![];
    bfs(1, &v, &mut visited, &mut ans);
    println!("{:?}", ans);
}

// initial_v=start
// スタートからつながっている経路を全て ans に追加
fn dfs(v: usize, g: &Vec<Vec<usize>>, visited: &mut Vec<bool>, ans: &mut Vec<usize>) {
    visited[v] = true;
    ans.push(v);

    for &next in g[v].iter() {
        if !visited[next] {
            dfs(next, g, visited, ans);
        }
    }
}

// initial_v=start
// スタートからの最短経路を ans に追加
fn bfs(v: usize, g: &Vec<Vec<usize>>, visited: &mut Vec<bool>, ans: &mut Vec<usize>) {
    let mut queue: Vec<usize> = vec![v];
    visited[v] = true;

    while !queue.is_empty() {
        let v = queue.pop().unwrap();
        ans.push(v);

        for &next in g[v].iter() {
            if !visited[next] {
                queue.push(next);
                visited[next] = true;
            }
        }
    }
}
