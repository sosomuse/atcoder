use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize
    };

    let mut count = 0;

    for a in 1..=n {
        for b in 1..=n {
            for c in 1..=n {
                if a < b && b < c && c <= n {
                    if a + b + c == x {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("{}", count);
}
