use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        _: usize,
    };

    let mut product = vec![];

    for _ in 0..n {
        input! {
            p: usize,
            c: usize,
            f: [usize; c],
        }
        product.push((p, c, f));
    }

    let mut features = vec![BTreeSet::new(); n];
    for i in 0..n {
        for f in product[i].2.clone() {
            features[i].insert(f);
        }
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let (pi, _, _) = product[i];
            let (pj, _, _) = product[j];

            if pi >= pj {
                if features[j].is_superset(&features[i])
                    && (pi > pj || features[i].len() < features[j].len())
                {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
