use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); n],
        cd: [(Usize1, Usize1); m],
    };

    let mut grid = vec![vec![false; w]; h];
    let mut blocks = vec![vec![false; w]; h];
    let mut vertical = vec![vec![false; w]; h];
    let mut horizontal = vec![vec![false; w]; h];

    for (c, d) in cd {
        blocks[c][d] = true;
    }

    for &(a, b) in ab.iter() {
        if vertical[a][b] {
            continue;
        }

        grid[a][b] = true;

        for i in (0..b).rev() {
            if blocks[a][i] {
                break;
            }

            grid[a][i] = true;
            vertical[a][i] = true;
        }

        for i in (b + 1)..w {
            if blocks[a][i] {
                break;
            }

            grid[a][i] = true;
            vertical[a][i] = true;
        }
    }

    for &(a, b) in ab.iter() {
        if horizontal[a][b] {
            continue;
        }

        grid[a][b] = true;

        for i in (0..a).rev() {
            if blocks[i][b] {
                break;
            }

            grid[i][b] = true;
            horizontal[i][b] = true;
        }

        for i in (a + 1)..h {
            if blocks[i][b] {
                break;
            }

            grid[i][b] = true;
            horizontal[i][b] = true;
        }
    }

    println!("{}", grid.into_iter().flatten().filter(|&x| x).count());
}
