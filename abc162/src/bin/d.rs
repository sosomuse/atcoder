use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut cnt = [0; 8];

    let s = s
        .into_iter()
        .map(|c| match c {
            'R' => 1,
            'G' => 2,
            'B' => 4,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    let mut ans = 0usize;
    for j in 0..n {
        for k in j + 1..n {
            if s[j] == s[k] {
                continue;
            }

            let si = 7 - s[j] - s[k];
            ans += cnt[si];

            let i = j as isize - (k as isize - j as isize) as isize;
            if i >= 0 {
                let msk = s[i as usize] | s[j] | s[k];
                if msk == 7 {
                    ans -= 1;
                }
            }
        }

        cnt[s[j]] += 1;
    }

    println!("{}", ans);
}
