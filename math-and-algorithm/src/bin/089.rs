use proconio::input;

fn main() {
    input! {
        a: u128,
        b: u64,
        c: u128,
    }

    if c == 1 {
        println!("No");
        return;
    }

    let mut x: u128 = c;

    for _ in 0..b {
        if a < x {
            println!("Yes");
            return;
        }
        x *= c;
    }

    println!("No");
}
