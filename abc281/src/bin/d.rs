use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        d: usize,
        a: [usize; n],
    };

    dbg!(n, k, d, a);

    println!("{}", n);
}
