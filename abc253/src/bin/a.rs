use proconio::input;

fn main() {
    input! {
        (a, b, c): (u32, u32, u32),
    }

    if b < c && b < a {
        println!("No");
        return;
    }

    if b > c && b > a {
        println!("No");
        return;
    }

    println!("Yes");
}
