use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        grid: [Chars; n],
    }

    for i in 0..n - 8 {
        for j in 0..m - 8 {
            let mut is_tak_code = true;

            for x in i..i + 3 {
                for y in j..j + 3 {
                    if grid[x][y] != '#' || grid[x + 6][y + 6] != '#' {
                        is_tak_code = false;
                    }
                }
            }

            for x in i..i + 3 {
                if grid[x][j + 3] == '#' {
                    is_tak_code = false;
                }
            }

            for x in i + 6..i + 9 {
                if grid[x][j + 5] == '#' {
                    is_tak_code = false;
                }
            }

            for y in j..j + 3 {
                if grid[i + 3][y] == '#' {
                    is_tak_code = false;
                }
            }

            for y in j + 6..j + 9 {
                if grid[i + 5][y] == '#' {
                    is_tak_code = false;
                }
            }

            if grid[i + 3][j + 3] == '#' || grid[i + 5][j + 5] == '#' {
                is_tak_code = false;
            }

            if is_tak_code {
                println!("{} {}", i + 1, j + 1);
            }
        }
    }
}
