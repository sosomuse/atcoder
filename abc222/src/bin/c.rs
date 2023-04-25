use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars; 2 * n],
    };

    let mut ans = vec![];

    for i in 0..2 * n {
        ans.push((0, i));
    }

    for i in 0..m {
        for j in 0..n {
            let x = ans[2 * j].1;
            let y = ans[2 * j + 1].1;

            let xv = a[x][i];
            let yv = a[y][i];

            if xv == yv {
                continue;
            }

            if (xv == 'G' && yv == 'C') || (xv == 'C' && yv == 'P') || (xv == 'P' && yv == 'G') {
                ans[2 * j].0 -= 1;
            } else {
                ans[2 * j + 1].0 -= 1;
            }
        }

        ans.sort();
    }

    for i in 0..2 * n {
        println!("{}", ans[i].1 + 1);
    }
}
