use proconio::input;

fn main() {
    input! {
        q: usize,
        lr: [(usize, usize); q],
    };

    let primes = sieve_of_eratosthenes(100_000);
    let mut s = vec![0; 100_001];

    for i in 1..=100_000 {
        if primes[i] && primes[(i + 1) / 2] {
            s[i] = s[i - 1] + 1;
        } else {
            s[i] = s[i - 1];
        }
    }

    for (l, r) in lr {
        println!("{}", s[r] - s[l - 1]);
    }
}

fn sieve_of_eratosthenes(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut i = 2;
    while i * i <= n {
        if is_prime[i] {
            for j in (i * i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }

        i += 1;
    }

    is_prime
}
