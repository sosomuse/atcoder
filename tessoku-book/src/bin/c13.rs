use std::collections::HashMap;

use proconio::input;

const MOD: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        p: usize,
        mut a: [usize; n],
    };

    for v in a.iter_mut() {
        *v %= MOD;
    }

    a.sort();

    let mut ans = 0;
    let mut count = HashMap::new();

    for i in 0..n {
        let v = a[i];
        if v == 0 {
            if p == 0 {
                ans += i;
            }
        } else {
            let goal = division(p, v, MOD);
            if let Some(&c) = count.get(&goal) {
                ans += c;
            }
        }

        *count.entry(v).or_insert(0) += 1;
    }

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
