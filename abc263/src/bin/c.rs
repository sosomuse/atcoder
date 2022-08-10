use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize
    }

    let vec = {
        let mut comb = (1..=m).combinations(n).collect::<Vec<Vec<usize>>>();
        for row in &mut comb {
            row.sort();
        }
        comb
    };

    for v in vec.iter() {
        for t in v {
            print!("{} ", t);
        }

        println!();
    }
}
