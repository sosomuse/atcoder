use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    let mut s = vec![0; n + 1];

    for i in 1..=n {
        s[i] = s[i - 1] + a[i - 1];
    }

    let mut r = vec![0; n];

    for i in 0..=n - 1 {
        if i == 0 {
            r[i] = 0;
        } else {
            r[i] = r[i - 1];
        }

        while r[i] < n && s[r[i] + 1] - s[i] <= k {
            r[i] += 1;
        }
    }

    let mut ans = 0;

    for i in 0..=n - 1 {
        ans += r[i] - i;
    }

    println!("{}", ans);
}
