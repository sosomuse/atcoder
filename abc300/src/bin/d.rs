use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut max = 0;
    while 2 * 2 * 3 * max * max <= n {
        max += 1;
    }
    let primes: Vec<usize> = solve(max);
    let len = primes.len();
    let mut ans = 0;

    for i in 0..len - 2 {
        let x = primes[i];
        let y = primes[i + 1];
        let z = primes[i + 2];
        if x * x * y * z * z > n {
            break;
        }

        let mut r = len - 1;

        for j in i + 1..len - 1 {
            let y = primes[j];
            let z = primes[j + 1];
            if x * x * y * z * z > n {
                break;
            }

            if r < j {
                break;
            }

            let t = x * x * primes[j];

            while r > j && t > n / primes[r] / primes[r] {
                r -= 1;
            }

            ans += r - j;
        }
    }
    println!("{}", ans);
}

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
