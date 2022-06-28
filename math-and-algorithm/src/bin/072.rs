use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
    };

    let solve = |t: isize| {
        let cl = (a + t - 1) / t;
        let cr = b / t;
        if cr - cl >= 1 {
            true
        } else {
            false
        }
    };

    let mut ans = 0;

    for i in 1..=b {
        if solve(i) {
            ans = i;
        }
    }

    println!("{}", ans);
}
