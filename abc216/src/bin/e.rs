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

        if diff == 0 {
            continue;
        }

        if diff * i >= k {
            let cnt = k / i;
            let rem = k % i;

            if cnt != 0 {
                let v = sum_range(a[i - 1], a[i - 1] - cnt + 1);
                ans += v * i;
            }

            ans += (a[i - 1] - cnt) * rem;
            k = 0;
            break;
        } else {
            let v = sum_range(a[i - 1], a[i] + 1);
            ans += v * i;
            k -= diff * i;
        }
    }

    if k > 0 {
        let cnt = k / n;
        let rem = k % n;
        ans += sum_range(a[n - 1], a[n - 1].saturating_sub(cnt) + 1) * n;
        ans += (a[n - 1].saturating_sub(cnt)) * rem;
    }

    println!("{}", ans);
}

fn sum_range(a: usize, b: usize) -> usize {
    let sum_a = a * (a + 1) / 2;
    let sum_b = b * (b - 1) / 2;

    sum_a - sum_b
}
