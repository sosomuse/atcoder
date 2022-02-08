use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    if -2i64.pow(31) <= n && 2i64.pow(31) > n {
        println!("Yes");
    } else {
        println!("No")
    }
}
