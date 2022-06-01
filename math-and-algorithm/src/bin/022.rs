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

    for (k, v) in hash_map.into_iter() {
        let target = 100000 - k;
        let count = &0;

        if *count > 0 {
            ans += v * count;
        }
    }

    println!("{}", ans);
}

fn binomial(n: usize, k: usize) -> usize {
    let mut ret = 1;
    for i in 0..k {
        ret *= n - i;
        ret /= i + 1;
    }

    return ret;
}
