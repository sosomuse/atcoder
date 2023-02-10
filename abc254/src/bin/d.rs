use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;

    for i in 1..=n {
        let v = i * i;
        ans += 1;
    }
}

// 素因数分解（√n）
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
            ans.push(p);
        }
    }

    if n != 1 {
        ans.push(n);
    }

    ans
}
