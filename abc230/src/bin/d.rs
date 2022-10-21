use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        mut lr: [(usize, usize); d],
    };

    lr.sort_by_key(|x| x.1);

    let mut destroyed = 0;
}
