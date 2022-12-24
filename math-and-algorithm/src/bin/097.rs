use ::proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    };

    let ans = solve(l, r);

    println!("{}", ans.len());
}

fn solve(min: usize, max: usize) -> Vec<usize> {
    let x = (max as f64).sqrt() as usize;
    let mut ans_flags = vec![true; max - min + 1];
    let mut flags = vec![true; x + 1];

    if min == 1 {
        ans_flags[0] = false;
    }

    for p in 2..=x {
        if flags[p] == false {
            continue;
        }

        let mut v = (min / p).max(1);

        if min % p != 0 {
            v += 1;
        }

        for m in ((p * v)..=max).step_by(p) {
            ans_flags[m - min] = false;
        }

        for m in (p..=x).step_by(p) {
            flags[m] = false;
        }
    }

    let mut ans: Vec<usize> = vec![];

    for (i, v) in ans_flags.iter().enumerate() {
        if *v {
            ans.push(i);
        }
    }

    dbg!(&ans);

    ans
}
