use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        c: char,
        a: Chars,
    };

    let mut score = 0;

    for i in 0..n {
        let v = a[i];

        match v {
            'W' => score += 0,
            'B' => score += 1,
            'R' => score += 2,
            _ => unreachable!(),
        }
    }

    let m = score % 3;

    if m == 0 && c == 'W' {
        println!("Yes");
    } else if m == 1 && c == 'B' {
        println!("Yes");
    } else if m == 2 && c == 'R' {
        println!("Yes");
    } else {
        println!("No");
    }
}
