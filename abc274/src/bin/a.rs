use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    };

    let ans = b / a;

    println!("{:.03}", ans);
}
