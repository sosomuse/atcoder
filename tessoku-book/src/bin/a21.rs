use proconio::input;

fn main() {
    input! {
        n: usize,
        pa: [(usize, usize); n],
    };

    let dp = vec![vec![0; n + 1]; n + 1];

    dbg!(dp, pa);
}
