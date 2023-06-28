use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };

    if a > b {
        println!("Takahashi");
    } else if b > a {
        println!("Aoki");
    } else {
        if c == 0 {
            println!("Aoki");
        } else {
            println!("Takahashi");
        }
    }
}
