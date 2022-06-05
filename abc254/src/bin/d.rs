use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut sqrt_set = std::collections::HashSet::<usize>::new();

    let mut ans: usize = 0;
    for i in 1..=n {
        let v = i.pow(2);
        sqrt_set.insert(v);
        ans += 1;
    }

    for s in sqrt_set {
        let primes = fnc(s, n);
        ans += primes;
    }

    println!("{}", ans);
}

fn fnc(n: usize, l: usize) -> usize {
    let mut vec = Vec::new();
    for i in 1..=(n as f64).sqrt() as usize {
        if n % i != 0 {
            continue;
        }

        if i != n / i && n / i <= l {
            vec.push(i);
            vec.push(n / i);
        }
    }
    return vec.len();
}
