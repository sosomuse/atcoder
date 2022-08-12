use proconio::input;

fn main() {
    input! {
        n:usize,
        mut m:usize,
        mut ab:[(usize,usize);n],
    };

    ab.sort_by_key(|&(a, _)| a);
    let mut ans = 0;

    for &(a, b) in &ab {
        if m > b {
            m -= b;
            ans += a * b;
        } else {
            ans += a * m.min(b);
            break;
        }
    }

    println!("{}", ans);
}
