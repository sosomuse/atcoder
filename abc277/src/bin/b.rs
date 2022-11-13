use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };

    let x = ['S', 'H', 'C', 'D'];
    let y = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];

    let mut ok = true;

    for i in 0..n {
        let v = &s[i];

        if !x.contains(&v[0]) || !y.contains(&v[1]) {
            ok = false;
        }
    }

    for i in 0..n {
        for j in i + 1..n {
            if s[i][1] == s[j][1] && s[i][0] == s[j][0] {
                ok = false;
            }
        }
    }

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
