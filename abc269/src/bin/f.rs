use proconio::input;

fn main() {
    input! {
        _: usize,
        m: usize,
        q: usize,
    };

    for _ in 0..q {
        input! {
            a: usize,
            b: usize,
            c: usize,
            d: usize,
        };

        let x = ((a - 1) * m + c) % 998244353;
        let y = (x + (d - c)) % 998244353;
        let z = b - a + 1;
        let w = d - c + 1;

        let x2 = (y * (y + 1) - (x - 1) * (x - 1 + 1)) / 2;
        let y2 = (x2 * z) % 998244353 + (z * w * m) % 998244353;

        let sum = z * w;
        // let ave = y2 / sum;

        if sum % 2 == 0 {
            println!("{}", y2 / 2);
        } else {
            // println!("{}", y2 / 2 - ave);
        }

        dbg!(y2, x2);
        dbg!(y2 / (z * w));
    }
}
