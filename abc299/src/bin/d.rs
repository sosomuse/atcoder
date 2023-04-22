use proconio::input;
use proconio::source::line::LineSource;
use std::io;
use std::io::{stdin, BufReader, Write};

fn main() {
    let stdin = stdin();
    let mut s: LineSource<BufReader<io::StdinLock>> = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut s,
        n: usize
    }

    let mut l = 0;
    let mut r = n;

    loop {
        let mid = (l + r) / 2;
        println!("? {}", mid + 1);
        io::stdout().flush().unwrap();

        input! {
            from &mut s,
            s_mid: usize,
        };

        if s_mid == 0 {
            l = mid;
        } else {
            r = mid;
        }

        if r - l == 1 {
            break;
        }
    }

    println!("! {}", r);
}
