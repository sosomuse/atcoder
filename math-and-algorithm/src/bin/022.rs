use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    let mut hash_map = std::collections::HashMap::<usize, usize>::new();

    for v in a {
        *hash_map.entry(v).or_insert(0) += 1;
    }

    for (k, v) in &hash_map {
        if *k >= 50000 {
            continue;
        }
        let target = 100000 - k;
        let count = hash_map.get(&target).unwrap_or(&0);

        if *count > 0 {
            ans += v * count;
        }
    }

    let mid = hash_map.get(&50000).unwrap_or(&0);
    ans += *mid * (mid - 1) / 2;

    println!("{}", ans);
}
