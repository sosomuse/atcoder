use proconio::input;

fn main() {
    input! {
        n: u32,
        a: [u32; n],
        b: [u32; n],
    }

    let mut c: u32 = 0;
    let mut d: u32 = 0;

    for i in 0..a.len() {
        for j in 0..b.len() {
            if a[i] == b[j] && i == j {
                c += 1;
            } else if a[i] == b[j] {
                d += 1;
            }
        }
    }

    println!("{}", c);
    println!("{}", d);
}
