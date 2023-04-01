use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [[isize; w]; h],
        b: [[isize; w]; h],
    };

    let mut ans = 0;

    for i in 0..h - 1 {
        for j in 0..w - 1 {
            if a[i][j] > b[i][j] {
                let diff = a[i][j] - b[i][j];
                ans += diff;

                for (di, dj) in [(0, 0), (0, 1), (1, 0), (1, 1)].iter() {
                    a[i + di][j + dj] -= diff;
                }
            } else {
                let diff = b[i][j] - a[i][j];
                ans += diff;

                for (di, dj) in [(0, 0), (0, 1), (1, 0), (1, 1)].iter() {
                    a[i + di][j + dj] += diff;
                }
            }
        }
    }

    if a == b {
        println!("Yes");
        println!("{}", ans);
    } else {
        println!("No");
    }
}
