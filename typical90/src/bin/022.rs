use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };

    let r = gcd(a, gcd(b, c));

    println!("{}", a / r + b / r + c / r - 3);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
