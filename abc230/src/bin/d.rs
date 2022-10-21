use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        mut lr: [(usize, usize); n],
    };

    lr.sort_by_key(|x| x.1);

    let mut ans = 0;
    let mut destroyed = 0;

    for (l, r) in lr {
        if l > destroyed {
            destroyed = r + d - 1;
            ans += 1;
        }
    }

    println!("{}", ans);
}
