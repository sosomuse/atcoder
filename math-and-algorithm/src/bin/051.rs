use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }

    let mut child = 1;
    let mut parent = 1;
    let _mod = 1000000007;

    for i in 1..=x + y {
        child = child * i % _mod;
    }

    for i in 1..=x {
        parent = parent * i % _mod;
    }

    for i in 1..=y {
        parent = parent * i % _mod;
    }

    println!("{}", division(child, parent, _mod))
}

fn division(a: usize, b: usize, m: usize) -> usize {
    return (a * modpow(b, m - 2, m)) % m;
}

fn modpow(a: usize, b: usize, m: usize) -> usize {
    let mut p = a;
    let mut ans = 1;

    for i in 0..30 {
        if b & (1 << i) != 0 {
            ans = ans * p % m;
        }
        p = p * p % m;
    }

    ans
}
