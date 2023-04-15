use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut a_count = 0;
    let mut c_count = 0;
    let mut prev = None;
    let mut counts = vec![];

    for i in 0..n {
        if prev == None && s[i] == 'A' {
            prev = Some('A');
            a_count += 1;
        } else if prev == Some('A') && s[i] == 'A' {
            a_count += 1;
        } else if prev == Some('A') && s[i] == 'R' {
            prev = Some('R');
        } else if prev == Some('R') && s[i] == 'C' {
            prev = Some('C');
            c_count += 1;
        } else if prev == Some('C') && s[i] == 'C' {
            c_count += 1;
        } else {
            if a_count > 0 && c_count > 0 {
                counts.push(a_count.min(c_count));
            }
            a_count = 0;
            c_count = 0;
            prev = None;

            if s[i] == 'A' {
                prev = Some('A');
                a_count += 1;
            }
        }
    }

    if a_count > 0 && c_count > 0 {
        counts.push(a_count.min(c_count));
    }

    let mut count1 = 0;
    let mut count2 = 0;

    for i in 0..counts.len() {
        let v = counts[i];
        if v == 1 {
            count1 += 1;
        } else {
            count2 += v - 2;
        }
    }

    let mut ans = counts.len() * 2;
    if count1 > count2 {
        ans -= count1 - count2;
    };

    println!("{}", ans);
}
