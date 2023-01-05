use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [isize; n],
        c: [isize; n],
    };

    let mut ans = 0;

    for i in 0..n {
        let x = v[i] - c[i];
        if x > 0 {
            ans += x;
        }
    }

    println!("{}", ans);
}
