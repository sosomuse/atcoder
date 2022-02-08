use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    if n >= 5 {
        println!("Yes");
        return;
    }

    if 2_i32.pow(n as u32) > n * n {
        println!("Yes");
    } else {
        println!("No")
    }
}
