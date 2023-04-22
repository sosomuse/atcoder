use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };

    let mut min = s.clone();
    let mut max = s.clone();

    for _ in 0..s.len() {
        let first = s.remove(0);
        s.push(first);

        if s < min {
            min = s.clone();
        }

        if s > max {
            max = s.clone();
        }
    }

    println!("{}", min.iter().collect::<String>());
    println!("{}", max.iter().collect::<String>());
}
