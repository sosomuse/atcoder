use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h1: usize,
        w1: usize,
        a: [[usize; w1]; h1],
        h2: usize,
        w2: usize,
        b: [[usize; w2]; h2],
    };

    let is_same = |x: &Vec<Vec<usize>>, y: &Vec<Vec<usize>>| {
        for i in 0..x.len() {
            for j in 0..x[0].len() {
                if x[i][j] != y[i][j] {
                    return false;
                }
            }
        }
        true
    };

    let h_diff = h1 - h2;
    let w_diff = w1 - w2;

    let mut cmb_h = (1..=h1).combinations(h_diff).collect::<Vec<Vec<usize>>>();
    let mut cmb_w = (1..=w1).combinations(w_diff).collect::<Vec<Vec<usize>>>();

    for v in cmb_h.iter_mut() {
        v.sort_by(|a, b| b.cmp(a));
    }

    for v in cmb_w.iter_mut() {
        v.sort_by(|a, b| b.cmp(a));
    }

    for h in cmb_h {
        let mut c = a.clone();

        for v in h {
            c.remove(v - 1);
        }

        for w in cmb_w.iter() {
            let mut d = c.clone();

            for v in w {
                for i in 0..d.len() {
                    d[i].remove(v - 1);
                }
            }

            if is_same(&d, &b) {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
