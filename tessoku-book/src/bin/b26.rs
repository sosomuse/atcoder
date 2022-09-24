use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    solve(n);
}

fn solve(n: usize) -> () {
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

    for (i, v) in flags.iter().enumerate() {
        if *v {
            println!("{}", i);
        }
    }
}
