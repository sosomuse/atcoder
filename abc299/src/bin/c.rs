use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut ans = -1;
    let mut count = 0;

    for i in 0..n {
        if s[i] == 'o' {
            count += 1;
        } else {
            if count > 0 {
                ans = std::cmp::max(ans, count as isize);
                count = 0;
            }
        }
    }

    let all_o = s.iter().all(|&x| x == 'o');

    if count > 0 && !all_o {
        ans = std::cmp::max(ans, count as isize);
    }

    println!("{}", ans);
}
