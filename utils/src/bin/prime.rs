fn main() {
    if is_prime(121) {
        println!("Yes");
    } else {
        println!("No");
    }

    let primes = solve(100);
    println!("{:?}", primes);
}

// O(√n)
fn is_prime(n: usize) -> bool {
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }

        i += 1;
    }

    return true;
}

// エラトステネスの篩
// O(n log log n)
fn solve(n: usize) -> Vec<usize> {
    let mut flags = vec![true; n + 1];
    flags[0] = false;
    flags[1] = false;

    let x = (n as f64).sqrt() as usize;
    for p in 2..=x {
        if !flags[p] {
            continue;
        }

        for m in ((p * p)..=n).step_by(p) {
            flags[m] = false;
        }
    }

    let mut ans: Vec<usize> = vec![];

    for (i, v) in flags.iter().enumerate() {
        if *v {
            ans.push(i);
        }
    }

    ans
}
