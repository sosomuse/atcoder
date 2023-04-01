use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars,
    };

    let mut ans = x.len();
    let mut s_count = 0;

    for i in 0..x.len() {
        if x[i] == 'S' {
            s_count += 1;
        } else {
            if s_count > 0 {
                s_count -= 1;
                ans -= 2;
            }
        }
    }

    println!("{}", ans);
}
