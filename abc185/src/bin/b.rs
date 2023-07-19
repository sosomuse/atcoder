use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        t: usize,
        ab: [(usize, usize); m],
    };

    let mut res = n;
    let mut prev = 0;

    for (a, b) in ab {
        res = res.saturating_sub(a - prev);
        if res == 0 {
            println!("No");
            return;
        }
        res += b - a;
        res = res.min(n);
        prev = b;
    }

    res = res.saturating_sub(t - prev);

    if res == 0 {
        println!("No");
        return;
    }

    println!("Yes");
}
