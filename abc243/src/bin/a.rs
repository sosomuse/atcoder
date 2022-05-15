use proconio::input;

fn main() {
    input! {
        (v, a, b, c): (u32,u32,u32,u32),
    }

    let t = v % (a + b + c);

    if t < a {
        println!("F");
    } else if t < a + b {
        println!("M");
    } else if t < a + b + c {
        println!("T");
    }
}
