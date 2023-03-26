use proconio::{input, marker::Chars};

fn main() {
    input! {
        r: usize,
        c: usize,
        mut grid: [Chars; r],
    }

    for i in 0..r {
        for j in 0..c {
            if grid[i][j].is_numeric() {
                let v = grid[i][j].to_digit(10).unwrap();
                grid[i][j] = '.';
                // マンハッタン距離で v 以内のマスを . にする

                for k in 0..r {
                    for l in 0..c {
                        if (i as i32 - k as i32).abs() + (j as i32 - l as i32).abs() <= v as i32
                            && grid[k][l] == '#'
                        {
                            grid[k][l] = '.';
                        }
                    }
                }
            }
        }
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}
