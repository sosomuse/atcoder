use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    let max_n = std::cmp::min(h, w);
    let mut count = vec![0; max_n];

    for i in 1..h {
        for j in 1..w {
            if c[i][j] != '#' {
                continue;
            }

            for n in (1..=max_n).rev() {
                if i + n >= h || j + n >= w {
                    continue;
                }
                if i < n || j < n {
                    continue;
                }

                let mut is_ok = true;

                for k in 1..=n {
                    let vec = vec![
                        (1 * k as isize, 1 * k as isize),
                        (1 * k as isize, -1 * k as isize),
                        (-1 * k as isize, 1 * k as isize),
                        (-1 * k as isize, -1 * k as isize),
                    ];

                    for (x, y) in vec {
                        if c[(i as isize + x) as usize][(j as isize + y) as usize] != '#' {
                            is_ok = false;
                            break;
                        }
                    }
                }

                if is_ok {
                    count[n - 1] += 1;
                    break;
                }
            }
        }
    }

    println!(
        "{}",
        count
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
