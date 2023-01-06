use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut a = vec![0; s.len() / 2 + 1];

    let mut prev = s[0];
    let mut count = 0;

    for i in 1..s.len() {
        let v = s[i];

        if prev == '5' && v == '2' {
            prev = v;
            continue;
        }

        if prev == '2' && v == '5' {
            count += 1;
        } else {
            a[count] += 1;
            count = 0;
        }

        prev = v;
    }

    a[count] += 1;

    let mut ans = 0;

    for i in 1..a.len() {
        let v = a[i];
        ans += (1 + i) * i / 2 * v;
    }

    println!("{}", ans);
}
