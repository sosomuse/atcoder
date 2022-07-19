use proconio::input;

fn main() {
    input! {
        abc: usize,
    };

    let a = abc / 100;
    let b = abc % 100 / 10;
    let c = abc % 10;

    let bca = b * 100 + c * 10 + a;
    let cab = c * 100 + a * 10 + b;
    println!("{}", abc + bca + cab)
}
