use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    let mut r = vec![0; n + 1];

    for i in 1..=n - 1 {
        if i == 1 {
            r[i] = 1;
        } else {
            r[i] = r[i - 1];
        }

        while r[i] < n && a[r[i]] - a[i - 1] <= k {
            r[i] = 1;
        }
    }

    let mut ans = 0;

    for i in 1..=n - 1 {
        ans += r[i] - i;
    }

    println!("{}", ans);
}
