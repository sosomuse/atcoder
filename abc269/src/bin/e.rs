use proconio::input;
use proconio::source::line::LineSource;
use std::io;
use std::io::{stdin, BufReader, Write};

// インタラクティブ問題
fn main() {
    // 標準入力
    let stdin = stdin();
    let mut s = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut s,
        n: usize
    }

    let mut u = 1;
    let mut d = n + 1;

    // 縦軸二分探索
    loop {
        let m = (u + d) / 2;

        println!("? {} {} {} {}", u, m - 1, 1, n);
        io::stdout().flush().unwrap();

        input! {
            from &mut s,
            t: usize,
        };

        if t == m - u {
            u = m;
        } else {
            d = m;
        }

        if u + 1 == d {
            break;
        }
    }

    let mut l = 1;
    let mut r = n + 1;

    // 横軸二分探索
    loop {
        let m = (l + r) / 2;

        println!("? {} {} {} {}", 1, n, l, m - 1);
        io::stdout().flush().unwrap();

        input! {
            from &mut s,
            t: usize,
        };

        if t == m - l {
            l = m;
        } else {
            r = m;
        }

        if l + 1 == r {
            break;
        }
    }

    println!("! {} {}", u, l);
}
