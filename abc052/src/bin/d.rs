use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        x: [usize; n],
    };

    let mut ans = 0;
    let mut current = x[0];

    for i in 1..n {
        let dist = x[i] - current;

        ans += (a * dist).min(b);
        current = x[i];
    }

    println!("{}", ans);
}
