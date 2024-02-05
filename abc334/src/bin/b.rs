use proconio::input;

fn main() {
    input! {
        a: isize,
        m: isize,
        l: isize,
        r: isize,
    };

    let lf: isize = {
        if a < l {
            let x = (l - a) / m;
            let mut y = a + x * m;
            if y != l {
                y += m;
            };

            y
        } else if a > l {
            let x = (a - l) / m;
            let y = a - x * m;
            y
        } else {
            l
        }
    };

    let rl: isize = {
        if a < r {
            let x = (r - a) / m;
            let y = a + x * m;
            y
        } else if a > r {
            let x = (a - r) / m;
            let mut y = a - x * m;
            if y != r {
                y -= m;
            };

            y
        } else {
            r
        }
    };

    let ans = (rl - lf) / m + 1;
    println!("{}", ans);
}
