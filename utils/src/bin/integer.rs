fn main() {
    let a = mod_pow(2, 4, 1000000007);
    println!("{}", a);
}

// 累乗のmodを求める
fn mod_pow(a: usize, b: usize, m: usize) -> usize {
    let mut r = 1;
    let mut p = a;
    for i in 0..64 {
        if (b >> i) & 1 == 1 {
            r = r * p % m;
        }
        p = p * p % m;
    }
    r
}
