use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        a: [usize; n],
    };

    let mut ans = 0;

    for v in a {
        if v < p {
            ans += 1;
        }
    }

    println!("{}", ans);
}
