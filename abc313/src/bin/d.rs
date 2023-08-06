use proconio::input;
use proconio::source::line::LineSource;
use std::io;
use std::io::{stdin, BufReader, Write};

use itertools::Itertools;

// インタラクティブ問題
fn main() {
    // 標準入力
    let stdin = stdin();
    let mut s = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut s,
        n: usize,
        k: usize
    }

    let mut group_a = vec![];
    let mut group_b = vec![];
    let mut count = 0;
    let mut last = (1..=k).collect::<Vec<usize>>();

    let mut cmp = (1..=n).combinations(k).collect::<Vec<Vec<usize>>>();
    cmp.sort();

    dbg!(&cmp);

    for c in cmp {
        if count + 1 == n {
            break;
        }

        let q = format!(
            "? {}",
            c.iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
        println!("{}", q);
        io::stdout().flush().unwrap();

        input! {
            from &mut s,
            o: usize,
        };

        let mut key = c[k - 1];
        if count == 0 {
            if o == 0 {
                group_a.push(key);
            } else {
                group_b.push(key);
            }
        } else {
        }
    }

    let vec = {
        if group_a.len() > group_b.len() {
            &group_a
        } else {
            &group_b
        }
    };

    let q = format!(
        "? {}",
        vec[..k]
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
    println!("{}", q);
    io::stdout().flush().unwrap();

    input! {
        from &mut s,
        o: usize,
    };

    let mut ans = vec![0; n];

    print!("! ");
    for v in vec {
        ans[v - 1] = o;
    }

    let vec = {
        if group_a.len() > group_b.len() {
            group_b
        } else {
            group_a
        }
    };
    for v in vec {
        ans[v - 1] = 1 - o;
    }

    for v in ans {
        print!("{} ", v);
    }
}
