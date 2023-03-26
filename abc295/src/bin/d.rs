use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut mp = std::collections::HashMap::new();
    let mut cnt = vec![0usize; 10];

    mp.insert(cnt.clone(), 1);

    for &nx in &s {
        cnt[nx as usize - '0' as usize] += 1;
        cnt[nx as usize - '0' as usize] %= 2;
        *mp.entry(cnt.clone()).or_insert(0) += 1;
    }

    let mut res = 0usize;
    for (_, &x) in &mp {
        res += x * (x - 1) / 2;
    }

    println!("{}", res);
}
