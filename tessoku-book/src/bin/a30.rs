use proconio::input;

fn main() {
    input! {
        n: usize,
        r: usize,
    };

    let ans = ncr(n, r, 1_000_000_007);

    println!("{}", ans);
}

fn power(a: usize, b: usize, m: usize) -> usize {
    let mut p = a;
    let mut ans = 1;

    for i in 0..30 {
        let w = 1 << i;
        if (b & w) != 0 {
            ans = ans * p % m;
        }
        p = p * p % m;
    }

    ans
}

fn division(a: usize, b: usize, m: usize) -> usize {
    (a * power(b, m - 2, m)) % m
}

fn ncr(n: usize, r: usize, m: usize) -> usize {
    let mut a = 1;
    let mut b = 1;

    for i in 1..=n {
        a = a * i % m;
    }

    for i in 1..=r {
        b = b * i % m;
    }

    for i in 1..=(n - r) {
        b = b * i % m;
    }

    division(a, b, m)
}
