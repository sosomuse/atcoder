use proconio::input;
use proconio::source::line::LineSource;
use std::io;
use std::io::{stdin, BufReader, Write};

fn main() {
    let stdin = stdin();
    let mut s = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut s,
        n: usize
    }

    let mut vec: Vec<usize> = vec![];

    for i in 1..=2 * n + 1 {
        vec.push(i);
    }

    loop {
        println!("{}", vec[0]);
        io::stdout().flush().unwrap();

        input! {
            from &mut s,
            o: usize,
        };

        if o == 0 {
            break;
        }

        vec = vec
            .iter()
            .filter(|&x| *x != o && vec[0] != *x)
            .map(|&x| x)
            .collect();
    }
}
