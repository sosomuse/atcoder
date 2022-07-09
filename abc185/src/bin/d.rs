use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; m],
    };

    a.sort();

    let mut p = 1000000000000000000;

    if m == 0 {
        println!("{}", 1);
        return;
    }

    for i in 0..m {
        if i == 0 {
            let diff = a[i] - 1;
            if diff >= 1 {
                p = p.min(diff).max(1);
            }
            continue;
        }

        if i == m - 1 {
            let diff = n - a[i];
            if diff >= 1 {
                p = p.min(diff).max(1);
            }
        }

        let diff = (a[i] - a[i - 1]) - 1;
        p = p.min(diff).max(1);
    }

    let mut ans = 0;

    for i in 0..m {
        if i == 0 {
            let diff = a[i] - 1;
            if diff >= 1 {
                ans += (diff as f64 / p as f64).ceil() as usize;
            }
            continue;
        }

        if i == m - 1 {
            let diff = n - a[i];
            if diff >= 1 {
                ans += (diff as f64 / p as f64).ceil() as usize;
            }
        }

        let diff = (a[i] - a[i - 1]) - 1;
        ans += (diff as f64 / p as f64).ceil() as usize;
    }

    println!("{}", ans);
}
