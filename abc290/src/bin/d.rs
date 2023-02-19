use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            d: usize,
            k: usize,
        }

        println!("{}", solve(n, d, k));
    }
}

fn solve(n: usize, d: usize, k: usize) -> usize {
    let x = d * (k - 1) % n;
    let y = (k - 1) / (n / gcd(n, d));

    x + y
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
