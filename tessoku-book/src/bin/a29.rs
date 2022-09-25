use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    println!("{}", power(a, b, 1000000007));
}

fn power(a: usize, b: usize, m: usize) -> usize {
    let mut p = a;
    let mut ans = 1;

    for i in 0..30 {
        let w = 1 << i;
        if (b & w) != 0 {
            ans = ans * p % m;
        }
        p = p * p % m;
    }

    ans
}
