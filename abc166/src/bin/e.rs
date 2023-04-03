use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut cnt = vec![0usize; n];
    let mut ans = 0;

    for i in 0..n {
        if i >= a[i] {
            ans += cnt[i - a[i]];
        }
        if i + a[i] < n {
            cnt[i + a[i]] += 1;
        }
    }

    println!("{}", ans);
}
