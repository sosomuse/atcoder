use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        k: usize,
    };

    let n = s.len();
    let mut cnt = vec![0; n + 1];

    for i in 0..n {
        if s[i] == '.' {
            cnt[i + 1] = cnt[i] + 1;
        } else {
            cnt[i + 1] = cnt[i];
        }
    }

    let mut ans = 0;
    let mut r = 0;

    // 尺取り法
    for l in 0..n {
        while r < n && cnt[r + 1] - cnt[l] <= k {
            r += 1;
        }

        ans = ans.max(r - l);
    }

    println!("{}", ans);
}
