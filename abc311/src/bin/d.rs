use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        grid: [Chars; n],
    }

    let mut visited = vec![vec![false; m]; n];
    let mut ans: Vec<Vec<bool>> = vec![vec![false; m]; n];
    let dx = vec![0, 0, -1, 1];
    let dy = vec![-1, 1, 0, 0];

    dfs(1, 1, &grid, &mut visited, &mut ans, &dx, &dy);

    let count = ans.into_iter().flatten().filter(|&x| x).count();

    println!("{}", count);
}

fn dfs(
    x: usize,
    y: usize,
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    ans: &mut Vec<Vec<bool>>,
    dx: &Vec<isize>,
    dy: &Vec<isize>,
) {
    if visited[x][y] {
        return;
    }
    visited[x][y] = true;
    ans[x][y] = true;

    for dir in 0..4 {
        let mut mag = 1;
        let mut nx = x as isize + dx[dir] * mag;
        let mut ny = y as isize + dy[dir] * mag;

        if nx < 0 || nx >= grid.len() as isize {
            continue;
        }

        if ny < 0 || ny >= grid[0].len() as isize {
            continue;
        }

        if grid[nx as usize][ny as usize] == '#' {
            continue;
        }

        loop {
            ans[nx as usize][ny as usize] = true;
            mag += 1;

            let nnx = x as isize + dx[dir] * mag;
            let nny = y as isize + dy[dir] * mag;

            if nnx < 0 || nnx >= grid.len() as isize {
                break;
            }

            if nny < 0 || nny >= grid[0].len() as isize {
                break;
            }

            if grid[nnx as usize][nny as usize] == '#' {
                break;
            }

            nx = nnx;
            ny = nny;
        }

        if mag == 1 {
            continue;
        }
        dfs(nx as usize, ny as usize, grid, visited, ans, dx, dy);
    }
}
