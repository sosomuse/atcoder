use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    };

    let mut ans = 0;

    for i in 0..n {
        for j in i + 1..n {
            let mut cnt = 0;

            for k in 0..m {
                if s[i][k] == 'o' || s[j][k] == 'o' {
                    cnt += 1;
                }
            }

            if cnt == m {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
