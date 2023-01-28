use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };

    let count = s.iter().filter(|&s| s == "For").count();

    if n / 2 < count {
        println!("Yes");
    } else {
        println!("No");
    }
}
