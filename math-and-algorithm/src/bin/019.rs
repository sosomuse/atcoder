use proconio::input;

fn main() {
    input! {
        n: usize,
        vec: [usize; n],
    }

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    for v in vec {
        if v == 1 {
            a += 1;
        } else if v == 2 {
            b += 1;
        } else if v == 3 {
            c += 1;
        }
    }

    println!("{}", binomial(a, 2) + binomial(b, 2) + binomial(c, 2));
}

fn binomial(n: usize, k: usize) -> usize {
    let mut ret = 1;
    for i in 0..k {
        ret *= n - i;
        ret /= i + 1;
    }

    return ret;
}
