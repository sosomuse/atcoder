use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        t: usize,
    };

    let ans = t / a * b;

    println!("{}", ans);
}
