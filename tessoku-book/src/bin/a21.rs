use proconio::input;

fn main() {
    input! {
        n: usize,
        pa: [(usize, usize); n],
    };

    let mut dp = vec![vec![0; n + 1]; n + 1];
}
