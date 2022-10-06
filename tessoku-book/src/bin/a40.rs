use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut map = std::collections::HashMap::<usize, usize>::new();

    for i in a {
        *map.entry(i).or_insert(0) += 1;
    }

    let mut ans = 0;

    for (_, v) in map {
        if v >= 3 {
            ans += binomial(v, 3)
        }
    }

    println!("{}", ans);
}

fn binomial(n: usize, k: usize) -> usize {
    let mut ret: usize = 1;
    for i in 0..k {
        ret *= n - i;
        ret /= i + 1;
    }

    return ret;
}
