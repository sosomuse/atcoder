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
    let mut no_set = BTreeSet::<(usize, usize)>::new();
    let mut set = BTreeSet::<(usize, usize)>::new();
    let mut set_index = vec![false; n];

    for i in 0..n {
        if i < k {
            set.insert((0, i));
            set_index[i] = true;
        } else {
            no_set.insert((0, i));
        }
    }

    for &(x, y) in &xy {
        let prev = a[x];

        // 使われている場合
        if set_index[x] {
            let (max, i) = *no_set.iter().next_back().unwrap_or(&(0, 0));
            // 前の値より大きい場合もしくは最大値より大きい場合
            if prev < y || max < y {
                set.remove(&(prev, x));
                set.insert((y, x));
                ans += y;
                ans -= prev;
            } else {
                set.remove(&(prev, x));
                no_set.insert((y, x));

                no_set.remove(&(max, i));
                set.insert((max, i));

                set_index[x] = false;
                set_index[i] = true;

                ans += max;
                ans -= prev;
            }
        } else {
            let (min, i) = *set.iter().next().unwrap();

            if min > y {
                no_set.remove(&(prev, x));
                no_set.insert((y, x));
            } else {
                set.remove(&(min, i));
                no_set.insert((min, i));

                no_set.remove(&(prev, x));
                set.insert((y, x));

                set_index[i] = false;
                set_index[x] = true;

                ans += y;
                ans -= min;
            }
        }

        a[x] = y;
        println!("{}", ans);
    }
}
