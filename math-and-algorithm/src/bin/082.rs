use proconio::input;
fn main() {
    input! {
        n: usize,
        mut lr: [(usize, usize); n],
    };

    lr.sort_by(|a, b| a.1.cmp(&b.1));

    let mut current_t = 0;
    let mut ans = 0;

    for (l, _) in lr {
        if current_t < l {
            ans += 1;
            current_t = l;
        }
    }

    println!("{}", ans);
}
