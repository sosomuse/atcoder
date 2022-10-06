use proconio::input;

fn main() {
    input! {
        n: usize,
        mut lr: [(usize, usize); n],
    };

    lr.sort_by_key(|&(_, r)| r);

    let mut start = 0;
    let mut count = 0;

    for (l, r) in lr {
        if start > l {
            continue;
        }

        start = r;
        count += 1;
    }

    println!("{}", count);
}
