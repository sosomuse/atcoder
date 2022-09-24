use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    println!("{}", gcd(a, b));
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
