use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let (primes, counts) = prime_factorize(n);

    let mut all_count = 0;

    for (_, count) in counts {
        all_count += count;
    }

    if primes.len() == 1 && all_count == 1 {
        println!("0");
        return;
    }

    let mut ans = 0;
    let mut p = 1;

    while p < all_count {
        p *= 2;
        ans += 1;
    }

    println!("{}", ans);
}

fn prime_factorize(mut n: usize) -> (Vec<usize>, std::collections::HashMap<usize, usize>) {
    let mut ans = vec![];
    let mut counts = std::collections::HashMap::new();

    for p in 2..=n {
        if p * p > n {
            break;
        }

        if n % p != 0 {
            continue;
        }

        let mut count = 0;

        while n % p == 0 {
            count += 1;
            n /= p;
        }

        ans.push(p);
        counts.insert(p, count);
    }

    if n > 1 {
        ans.push(n);
        counts.insert(n, 1);
    }

    (ans, counts)
}
