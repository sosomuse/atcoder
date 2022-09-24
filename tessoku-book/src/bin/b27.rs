use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    println!("{}", lcm(a, b));
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

fn lcm(a: usize, b: usize) -> usize {
    return a * b / gcd(a, b);
}
