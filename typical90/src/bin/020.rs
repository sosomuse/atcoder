use proconio::input;

fn main() {
    input! {
        a: usize,
        b: u32,
        c: usize,
    };

    let ans = a < c.pow(b);

    println!("{}", if ans { "Yes" } else { "No" });
}
