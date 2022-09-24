use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    for _ in 0..q {
        input! {
            x: usize,
        };

        println!("{}", if is_prime(x) { "Yes" } else { "No" });
    }
}

fn is_prime(n: usize) -> bool {
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }

        i += 1;
    }

    return true;
}
