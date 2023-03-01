use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut ans = std::usize::MAX;
    for bit in 0..(1 << (n - 1)) {
        let mut or = 0;
        let mut xor = 0;

        for i in 0..n {
            or |= a[i];
            if bit & (1 << i) != 0 {
                xor ^= or;
                or = 0;
            }
        }
        xor ^= or;
        ans = ans.min(xor);
    }

    println!("{}", ans);
}
