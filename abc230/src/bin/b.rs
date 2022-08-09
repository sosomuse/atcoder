use proconio::input;

fn main() {
    input! {
        s: String,
    };

    let x = "oxx";
    let y = x.repeat(10_usize.pow(5));

    if y.contains(&s) {
        println!("Yes");
    } else {
        println!("No");
    }
}
