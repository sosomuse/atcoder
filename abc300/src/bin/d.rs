use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut max = 0;
    while 2 * 2 * 3 * max * max <= n {
        max += 1;
    }
    let mut primes: Vec<usize> = solve(max);

    primes.sort();
    let len = primes.len();
    let mut ans = 0;

    for i in 0..len {
        let x = primes[i];
        if x * x * x * x * x > n {
            break;
        }

        let mut r = len - 1;

        for j in i + 1..len {
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
