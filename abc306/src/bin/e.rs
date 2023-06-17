use proconio::input;
use proconio::marker::Usize1;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize, k: usize, q: usize,
        xy: [(Usize1, usize); q],
    }

    let mut a = vec![0; n];
    let mut ans = 0;
    let mut unused_values = BTreeSet::<(usize, usize)>::new();
    let mut used_values = BTreeSet::<(usize, usize)>::new();
    let mut used = vec![false; n];

    for i in 0..n {
        if i < k {
            used_values.insert((0, i));
            used[i] = true;
        } else {
            unused_values.insert((0, i));
        }
    }

    for &(x, y) in &xy {
        let prev = a[x];

        // 使われている場合
        if used[x] {
            let (max, i) = *unused_values.iter().next_back().unwrap_or(&(0, 0));
            // 前の値より大きい場合もしくは最大値より大きい場合
            if prev < y || max < y {
                used_values.remove(&(prev, x));
                used_values.insert((y, x));
                ans += y;
                ans -= prev;
            } else {
                used_values.remove(&(prev, x));
                unused_values.insert((y, x));

                unused_values.remove(&(max, i));
                used_values.insert((max, i));

                used[x] = false;
                used[i] = true;

                ans += max;
                ans -= prev;
            }
        } else {
            let (min, i) = *used_values.iter().next().unwrap();

            if min > y {
                unused_values.remove(&(prev, x));
                unused_values.insert((y, x));
            } else {
                used_values.remove(&(min, i));
                unused_values.insert((min, i));

                unused_values.remove(&(prev, x));
                used_values.insert((y, x));

                used[i] = false;
                used[x] = true;

                ans += y;
                ans -= min;
            }
        }

        a[x] = y;
        println!("{}", ans);
    }
}
