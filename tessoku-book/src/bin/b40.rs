use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut cnt = std::collections::HashMap::<usize, usize>::new();

    for i in a {
        let v = i % 100;
        *cnt.entry(v).or_insert(0) += 1;
    }

    let mut ans = 0;

    for i in 1..=49 {
        let v = cnt.get(&i).unwrap_or(&0);
        let w = cnt.get(&(100 - i)).unwrap_or(&0);
        ans += w * v;
    }

    ans += binomial(*cnt.get(&0).unwrap_or(&0), 2);
    ans += binomial(*cnt.get(&50).unwrap_or(&0), 2);

    println!("{}", ans);
}

fn binomial(n: usize, k: usize) -> usize {
    if n < k {
        return 0;
    }

    let mut ret: usize = 1;
    for i in 0..k {
        ret *= n - i;
        ret /= i + 1;
    }

    return ret;
}
