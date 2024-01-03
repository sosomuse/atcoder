use proconio::input;

fn main() {
    input! {
        b: usize,
        g: usize,
    };

    if b < g {
        println!("Glove");
    } else {
        println!("Bat");
    }
}
