use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    let g = gcd(a, b);
    let ans = prime_factorize(g).len() + 1;

    println!("{}", ans);
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    let mut r = a % b;
    while r != 0 {
        a = b;
        b = r;
        r = a % b;
    }

    return b;
}

// 素因数分解
fn prime_factorize(mut n: usize) -> Vec<usize> {
    let mut ans = vec![];

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

        ans.push(p);
    }

    if n != 1 {
        ans.push(n);
    }

    ans
}
