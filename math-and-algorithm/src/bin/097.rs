use ::proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    };

    let mut ans = 0;
    let mut primes = vec![true; r - l + 1];

    let mut i = 3;
    while i * i <= l {
        for j in l..=r {
            if j == 1 {
                primes[j - l] = false;
                continue;
            }
            if j == 2 {
                primes[j - l] = true;
                continue;
            }
            if j % i == 0 {
                primes[j - l] = false;
                continue;
            }
        }
        i += 2;
    }

    println!("{}", ans);
}

fn is_prime(n: usize) -> bool {
    if n == 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}
