use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
        b: [usize; m],
    };

    let mut at = vec![0; n + 1];
    let mut bt = vec![0; m + 1];

    for i in 0..n {
        at[i + 1] = at[i] + a[i];
    }
    for i in 0..m {
        bt[i + 1] = bt[i] + b[i];
    }

    let mut ans = 0;
    let mut ok = m;

    for i in 0..=n {
        if at[i] > k {
            break;
        }
        while bt[ok] > k - at[i] {
            ok -= 1;
        }
        ans = ans.max(i + ok);
    }

    println!("{}", ans);
}
