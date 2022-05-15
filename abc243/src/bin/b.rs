use proconio::input;

fn main() {
    input! {
        n: u32,
        a: [u32; n],
        b: [u32; n],
    }

    let mut c: u32 = 0;
    let mut d: u32 = 0;

    for (v, i) in a.into_iter().enumerate() {
        for (s, j) in b.into_iter().enumerate() {
            if v == s && i == j {
                c += 1;
            } else if v == s {
                d += 1;
            }
        }
    }

    println!("{}", c);
    println!("{}", d);
}
