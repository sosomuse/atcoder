use proconio::input;

fn f(a: u64, b: u64) -> u64 {
    a * a * a + a * a * b + a * b * b + b * b * b
}

fn main() {
    input! {
        n: u64,
    };

    let mut k = 1000000;
    let mut x = std::u64::MAX;

    for i in 0..=1000000 {
        while f(i, k) >= n {
            x = x.min(f(i, k));

            if k == 0 {
                break;
            };

            k -= 1;
        }
    }

    println!("{}", x);
}
