use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };

    let mut ans = 0;

    let mut solve = |c: char, p: usize| {
        while s[p] != c {
            for i in 0..s.len() {
                if s[i] == c {
                    ans += 1;
                    s.swap(i, i - 1);
                }
            }
        }
    };

    solve('a', 0);
    solve('t', 1);
    solve('c', 2);
    solve('o', 3);
    solve('d', 4);
    solve('e', 5);
    solve('r', 6);

    println!("{}", ans);
}
