use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(Usize1, Usize1); m],
    };

    // 隣接リスト
    let mut graph = vec![vec![]; n];
    // 入次数
    let mut deg = vec![0; n];
    for (x, y) in xy {
        graph[y].push(x);
        deg[x] += 1;
    }

    let mut queue = std::collections::VecDeque::new();
    for i in 0..n {
        if deg[i] == 0 {
            queue.push_back(i);
        }
    }

    let mut na = n;
    let mut a = vec![na; n];
    while let Some(v) = queue.pop_front() {
        if !queue.is_empty() {
            println!("No");
            return;
        }

        a[v] = na;
        na -= 1;

        for &u in &graph[v] {
            deg[u] -= 1;
            if deg[u] == 0 {
                queue.push_back(u);
            }
        }
    }

    println!("Yes");
    for i in 0..n {
        print!("{} ", a[i]);
    }
}
