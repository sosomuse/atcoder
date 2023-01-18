use proconio::input;

fn main() {
    input! {
        n: usize,
        (sx, sy): (isize, isize),
        (tx, ty): (isize, isize),
        xyr: [(isize, isize, isize); n],
    }

    // sがどの円の中にあるか, tがどの円の中にあるか
    let mut s = None;
    let mut t = None;

    for i in 0..n {
        let (x, y, r) = xyr[i];
        if (x - sx).pow(2) + (y - sy).pow(2) == r.pow(2) {
            s = Some(i);
        }
        if (x - tx).pow(2) + (y - ty).pow(2) == r.pow(2) {
            t = Some(i);
        }
    }

    let mut graph = vec![vec![]; n];

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let (x1, y1, r1) = xyr[i];
            let (x2, y2, r2) = xyr[j];

            if is_circles_touching(
                x1 as f64, y1 as f64, r1 as f64, x2 as f64, y2 as f64, r2 as f64,
            ) {
                graph[i].push(j);
            }
        }
    }

    let ans = dfs(s.unwrap(), &graph);

    if ans.contains(&t.unwrap()) {
        println!("Yes");
    } else {
        println!("No");
    }
}

// 円が接しているかどうか
fn is_circles_touching(x1: f64, y1: f64, r1: f64, x2: f64, y2: f64, r2: f64) -> bool {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let distance = dx * dx + dy * dy;
    let r = r1 + r2;
    distance <= r * r && (distance.sqrt() + f64::min(r1, r2)).abs() >= f64::max(r1, r2)
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
