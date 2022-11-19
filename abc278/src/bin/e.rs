use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        h1: usize,
        w1: usize,
        _: usize,
        h2: usize,
        w2: usize,
        hw: [[usize; w1]; h1],
    }

    let count_h = h1 - h2 + 1;
    let count_w = w1 - w2 + 1;

    let mut counts = HashMap::new();
    let mut blacks = HashMap::new();

    for i in 0..h1 {
        for j in 0..w1 {
            let v = hw[i][j];
            *counts.entry(v).or_insert(0) += 1;
        }
    }

    let mut initial_counts = HashMap::new();
    for i in 0..h2 {
        for j in 0..w2 {
            let v = hw[i][j];
            *initial_counts.entry(v).or_insert(0) += 1;
        }
    }

    blacks.insert((0, 0), initial_counts);

    for i in 1..count_h {
        let b = blacks.get(&(i - 1, 0)).unwrap();
        let mut clone = b.clone();

        for j in 0..w2 {
            let v = hw[i - 1][j];
            *clone.entry(v).or_insert(0) -= 1;
        }

        for j in 0..w2 {
            let v = hw[i + h2 - 1][j];
            *clone.entry(v).or_insert(0) += 1;
        }

        blacks.insert((i, 0), clone);
    }

    for i in 0..count_h {
        for j in 1..count_w {
            let b = blacks.get(&(i, j - 1)).unwrap();
            let mut clone = b.clone();

            for k in 0..h2 {
                let v = hw[i + k][j - 1];
                *clone.entry(v).or_insert(0) -= 1;
            }

            for k in 0..h2 {
                let v = hw[i + k][j - 1 + w2];
                *clone.entry(v).or_insert(0) += 1;
            }

            blacks.insert((i, j), clone);
        }
    }

    for i in 0..count_h {
        for j in 0..count_w {
            let mut clone = counts.clone();

            let b = blacks.get(&(i, j)).unwrap();

            for (k, v) in b.iter() {
                if *v > 0 {
                    *clone.entry(*k).or_insert(0) -= v;
                }
            }

            let mut count = 0;

            for v in clone.values() {
                if *v > 0 {
                    count += 1;
                }
            }

            print!("{} ", count);
        }

        println!();
    }
}
