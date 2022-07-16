use proconio::input;

fn main() {
    input! {
        n: f64,
    };

    let x = n.powf(0.333334) as usize;
    let primes = solve(x);
    let mut ans = 0;

    for i in 0..primes.len() {
        let v = primes[i];

        for j in i + 1..primes.len() {
            let w = primes[j];
            if v * w * w <= n as usize / w {
                ans += 1;
            }
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
