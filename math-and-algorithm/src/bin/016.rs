use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = a[0];

    for i in 1..a.len() {
        let v = a[i];
        ans = gcd(ans, v);
    }

    println!("{}", ans);
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    let mut r = a % b;
    while r != 0 {
        a = b;
        b = r;
        r = a % b;
    }

    return b;
}
