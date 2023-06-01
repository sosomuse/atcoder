use proconio::input;

fn main() {
    input! {
        n: usize,
        apx: [(usize, usize, usize); n],
    };

    let mut ans = std::usize::MAX;
    for (a, p, x) in apx {
        if x > a {
            ans = ans.min(p);
        }
    }
    println!(
        "{}",
        if ans == std::usize::MAX {
            -1
        } else {
            ans as isize
        }
    )
}
