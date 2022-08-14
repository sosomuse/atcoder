use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
    };

    let mut grid = vec![vec![0; 16]; 16];

    for i in 2..=14 {
        grid[2][i] = 1;
        grid[14][i] = 1;
        grid[i][2] = 1;
        grid[i][14] = 1;
    }

    for i in 4..=12 {
        grid[4][i] = 1;
        grid[12][i] = 1;
        grid[i][4] = 1;
        grid[i][12] = 1;
    }

    for i in 6..=10 {
        grid[6][i] = 1;
        grid[10][i] = 1;
        grid[i][6] = 1;
        grid[i][10] = 1;
    }

    grid[8][8] = 1;

    let ans = grid[r][c];

    println!("{}", if ans == 0 { "black" } else { "white" });
}
