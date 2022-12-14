use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };

    let mut ok: Vec<bool> = vec![true; 1000001];
    a.sort();
    let mut map: HashMap<usize, usize> = HashMap::new();

    for i in 0..n {
        let v = a[i];

        *map.entry(a[i]).or_insert(0) += 1;

        if map[&a[i]] > 1 {
            ok[a[i]] = false;
        }

        if ok[v] {
            for j in 2.. {
                let k = v * j;
                if k >= 1000001 {
                    break;
                }
                ok[k] = false;
            }
        }
    }

    let mut ans: usize = 0;

    for i in 0..n {
        if ok[a[i]] {
            ans += 1;
        }
    }

    println!("{}", ans);
}
