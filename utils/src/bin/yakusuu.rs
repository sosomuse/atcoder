fn main() {
    let a = divisors(280);
    println!("{:?}", a);

    let b = prime_factorize(280);
    println!("{:?}", b);
}

// 約数の列挙
fn divisors(n: usize) -> Vec<usize> {
    let mut lst: Vec<usize> = vec![];

    let mut i = 1;

    while i * i <= n {
        if n % i == 0 {
            lst.push(i);
            if i != n / i {
                lst.push(n / i);
            }
        }

        i += 1;
    }

    lst
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
        }

        ans.push(p);
    }

    if n != 1 {
        ans.push(n);
    }

    ans
}
