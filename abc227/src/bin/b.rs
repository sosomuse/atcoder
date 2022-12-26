use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
    };

    let mut ans = 0;

    for v in s {
        'outer: for a in 1..300 {
            for b in 1..300 {
                if 4 * a * b + 3 * a + 3 * b == v {
                    ans += 1;
                    break 'outer;
                }
            }
        }
    }

    println!("{}", n - ans);
}
