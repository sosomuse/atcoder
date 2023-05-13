use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };

    let mut counts = vec![0; n + 1];
    let primes = solve(n);

    for p in primes {
        let mut j = p;

        while j <= n {
            counts[j] += 1;
            j += p;
        }
    }

    let mut ans = 0;
    for c in counts {
        if c >= k {
            ans += 1;
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
