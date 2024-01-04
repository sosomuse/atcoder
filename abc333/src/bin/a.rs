use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let s = n.to_string();

    println!("{}", s.repeat(n))
}
