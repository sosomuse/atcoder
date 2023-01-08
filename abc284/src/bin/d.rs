use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
        }

        let a = prime_factorize(n);
        let b = n / (a * a);

        if a * a * b == n {
            print!("{} {}", a, b);
        } else {
            let c = n / a;
            print!("{} {}", (c as f64).sqrt(), a);
        }

        println!();
    }
}

fn prime_factorize(mut n: usize) -> usize {
    for p in 2..=n {
        if p * p > n {
            break;
        }

        if n % p != 0 {
            continue;
        }

        while n % p == 0 {
            n /= p;
        }

        return p;
    }

    return n;
}
