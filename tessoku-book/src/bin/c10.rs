use proconio::input;

fn main() {
    input! {
        w: usize,
    };

    let m = 1000000007;
    println!("{}", 12 * power(7, w - 1, m) % m);
}

fn power(a: usize, b: usize, m: usize) -> usize {
    let mut p = a;
    let mut ans = 1;

    for i in 0..60 {
        let w = 1 << i;
        if (b & w) != 0 {
            ans = ans * p % m;
        }
        p = p * p % m;
    }

    ans
}
