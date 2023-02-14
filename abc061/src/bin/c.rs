use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut ab: [(usize, usize); n],
    }

    ab.sort_by_key(|&(a, _)| a);
    let mut count = 0;

    for (a, b) in ab {
        count += b;

        if count >= k {
            println!("{}", a);
            return;
        }
    }
}
