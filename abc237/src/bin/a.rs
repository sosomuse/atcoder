use proconio::input;

fn main() {
    input! {
        n: i128,
    };

    if n >= -2_i128.pow(31) && n < 2_i128.pow(31) {
        println!("Yes");
    } else {
        println!("No");
    }
}
