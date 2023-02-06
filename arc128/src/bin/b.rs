use proconio::input;
use std::cmp::min;

const MAX: isize = 1000000000;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            r: isize,
            g: isize,
            b: isize,
        };

        let mut ans = MAX;

        if (r - g) % 3 == 0 {
            ans = min(ans, r.max(g));
        };

        if (r - b) % 3 == 0 {
            ans = min(ans, r.max(b));
        };

        if (g - b) % 3 == 0 {
            ans = min(ans, g.max(b));
        };

        if ans == MAX {
            ans = -1;
        };

        println!("{}", ans);
    }
}
