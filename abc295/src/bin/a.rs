use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [String; n],
    }

    for s in &w {
        if s == "and" || s == "not" || s == "that" || s == "the" || s == "you" {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
