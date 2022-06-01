use proconio::input;

fn main() {
    input! {
        n: usize,
        vec: [usize; n],
    }

    let mut a: u64 = 0;
    let mut b: u64 = 0;
    let mut c: u64 = 0;
    let mut d: u64 = 0;

    for v in vec {
        if v == 100 {
            a += 1
        }
        if v == 200 {
            b += 1
        }
        if v == 300 {
            c += 1
        }
        if v == 400 {
            d += 1
        }
    }

    println!("{}", a * d + b * c);
}
