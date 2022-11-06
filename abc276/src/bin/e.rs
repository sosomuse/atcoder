use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Chars; h],
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; h * w + 1];

    let is_moving = |c: char| match c {
        '.' => true,
        'S' => true,
        _ => false,
    };

    let get_pos = |x: usize, y: usize| (x * w + y) as usize;
    let mut s = 0;

    for i in 0..h {
        for j in 0..w {
            let g = grid[i][j];

            if g == 'S' {
                s = get_pos(i, j);
            }

            if is_moving(g) {
                let pos = get_pos(i, j);
                if i > 0 {
                    let up = grid[i - 1][j];
                    if is_moving(up) {
                        graph[pos].push(get_pos(i - 1, j));
                    }
                }

                if j > 0 {
                    let left = grid[i][j - 1];
                    if is_moving(left) {
                        graph[pos].push(get_pos(i, j - 1));
                    }
                }

                if i + 1 < h {
                    let down = grid[i + 1][j];
                    if is_moving(down) {
                        graph[pos].push(get_pos(i + 1, j));
                    }
                }

                if j + 1 < w {
                    let right = grid[i][j + 1];

                    if is_moving(right) {
                        graph[pos].push(get_pos(i, j + 1));
                    }
                }
            }
        }
    }

    // dbg!(s, &graph);
    dfs(s, &graph, s);

    println!("No");
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, goal: usize) -> () {
    let mut visited: Vec<bool> = vec![false; graph.len()];
    dfs_inner(v, graph, &mut visited, goal, None);
}

fn dfs_inner(
    v: usize,
    graph: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    goal: usize,
    prev: Option<usize>,
) {
    visited[v] = true;

    for &w in graph[v].iter() {
        if w == goal && prev != Some(goal) {
            println!("Yes");
            std::process::exit(0);
        }

        if !visited[w] {
            dfs_inner(w, graph, visited, goal, Some(v));
        }
    }
}
