use proconio::input;

fn main() {
    input! {
        n: usize,
        r: usize,
    }

    println!("{}", binomial(n, r));
}

fn binomial(n: usize, k: usize) -> usize {
    let mut ret = 1;
    for i in 0..k {
        ret *= n - i;
        ret /= i + 1;
    }

    return ret;
}
