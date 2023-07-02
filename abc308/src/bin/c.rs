use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        ab: [(u64, u64); n],
    }

    let mut rates: Vec<(u64, u64, usize)> = Vec::new();
    for (i, &(a, b)) in ab.iter().enumerate() {
        rates.push((a, b, i + 1));
    }

    rates.sort_by(|a, b| {
        if b.1 == 0 {
            if a.1 == 0 {
                a.2.cmp(&b.2)
            } else {
                Ordering::Greater
            }
        } else if a.1 == 0 {
            Ordering::Less
        } else {
            (b.0 * a.1).cmp(&(a.0 * b.1))
        }
    });

    for (_, _, i) in rates {
        println!("{}", i);
    }
}
