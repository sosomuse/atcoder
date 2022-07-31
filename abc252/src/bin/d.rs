use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };

    a.sort();

    let mut map = HashMap::<usize, usize>::new();
    let mut ans = 0;
    let mut cnt = 0;

    for i in 0..n {
        let v = a[i];
        *map.entry(v).or_insert(0) += 1;
    }

    dbg!(ncr(4, 2));

    for _ in 0..map.keys().len() - 2 {
        let v = a[cnt];

        let c = *map.get(&v).unwrap_or(&1);

        dbg!(ans);

        ans += ncr(n - cnt - c, 2) * c;
        cnt += c;
    }

    println!("{}", ans);
}

fn ncr(n: usize, r: usize) -> usize {
    let mut _n = 1;
    let mut _r = 1;

    for i in 0..r {
        _n *= n - i;
        _r *= i + 1;
    }

    _n / _r
}
