use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Chars; h],
    }

    let dx = vec![-1, 0, 1, 0];
    let dy = vec![0, 1, 0, -1];
    let snuke = vec!['s', 'n', 'u', 'k', 'e'];
    let mut visited = vec![vec![vec![false; w]; h]; 5];
    let mut queue = VecDeque::new();

    queue.push_back((0, 0, 0));
    while let Some((x, y, depth)) = queue.pop_front() {
        let char_index = depth % 5;
        if visited[char_index][x][y] {
            continue;
        }

        if grid[x][y] != snuke[char_index] {
            continue;
        }

        visited[char_index][x][y] = true;

        if x == h - 1 && y == w - 1 {
            println!("Yes");
            return;
        }

        for i in 0..4 {
            let nx = x as i32 + dx[i];
            let ny = y as i32 + dy[i];
            if nx >= 0 && nx < h as i32 && ny >= 0 && ny < w as i32 {
                queue.push_back((nx as usize, ny as usize, depth + 1));
            }
        }
    }

    println!("No");
}
