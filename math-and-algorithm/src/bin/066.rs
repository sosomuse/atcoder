use proconio::input;

fn main() {
    input! {
        n: isize,
        k: isize,
    };

    let mut v = 0;

    for a in 1..=n {
        for b in 1.max(a - (k - 1))..=n.min(a + (k - 1)) {
            for c in 1.max(a - (k - 1))..=n.min(a + (k - 1)) {
                if (b - c).abs() <= k - 1 {
                    v += 1;
                }
            }
        }
    }

    println!("{}", n.pow(3) - v);
}
