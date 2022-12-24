use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        ab: [(usize, usize); m],
    };

    dbg!(n, m, k, ab);
}
