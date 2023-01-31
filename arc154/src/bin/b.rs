use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        mut t: Chars,
    };

    if s == t {
        println!("0");
        return;
    }

    let mut s_c = s.clone();
    let mut t_c = t.clone();
    s_c.sort();
    t_c.sort();

    if s_c != t_c {
        println!("-1");
        return;
    }

    t.reverse();
    let mut ans = n;

    for i in 0..n {
        if s[ans - 1] == t[i] {
            ans -= 1;
        }
    }

    println!("{}", ans);
}
