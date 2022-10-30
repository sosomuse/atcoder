use proconio::input;

fn main() {
    input! {
        mut a: u128,
        mut b: u128,
        mut c: u128,
        mut d: u128,
        mut e: u128,
        mut f: u128,
    };

    let m = 998244353;

    a = a % m;
    b = b % m;
    c = c % m;
    d = d % m;
    e = e % m;
    f = f % m;

    let x = (a * b * c) % m + m;
    let y = (d * e * f) % m;

    println!("{}", (x - y) % m);
}
