use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
        xy: [(isize, isize); n],
    }

    let mut ans: f64 = 0.;

    for (j, &(x1, y1)) in xy.iter().enumerate() {
        if a.contains(&(j + 1)) {
            continue;
        }

        let mut max: f64 = 10000000000.;

        for v in a.iter() {
            let t = *v - 1;
            let (x2, y2) = xy[t];
            let d = fnc(x1, x2, y1, y2);

            max = max.min(d);
        }

        ans = ans.max(max)
    }

    println!("{}", ans);
}

fn fnc(x1: isize, x2: isize, y1: isize, y2: isize) -> f64 {
    (((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt()
}
