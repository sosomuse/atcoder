use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut a: [usize; n],
    };

    a.sort_by(|a, b| b.cmp(a));

    let mut ans = 0;

    for i in 1..n {
        let diff = a[i - 1] - a[i];

        dbg!(i, diff, k, ans);

        if diff * i >= k {
            let cnt = k / i;
            let rem = k % i;
            let mut v = (a[i - 1] + a[i - 1] - cnt) / 2 * cnt;
            if cnt % 2 == 0 {
                v += 1;
            }

            ans += v * i;
            ans += (a[i - 1] - cnt - 1) * rem;
            k = 0;
            break;
        } else {
            let mut v = (a[i - 1] + a[i - 1] - diff) / 2 * diff;
            if diff % 2 == 0 {
                v += 1;
            };
            ans += v * i;
            k -= diff * i;
        }
    }

    if k > 0 {
        let cnt = k / n;
        let rem = k % n;
        ans += (a[n - 1] + a[n - 1] - cnt) / 2 * n;
        ans += (a[n - 1] - cnt - 1) * rem;
    }

    println!("{}", ans);
}
