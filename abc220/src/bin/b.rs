use proconio::input;

fn main() {
    input! {
        k: u32,
        a: String,
        b: String,
    };

    let x = u64::from_str_radix(&a, k).unwrap();
    let y = u64::from_str_radix(&b, k).unwrap();

    println!("{}", x * y);
}
