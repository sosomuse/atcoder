use proconio::input;

fn main() {
    input! {
        x: i64,
    }

    println!("{}", x.div_euclid(10));
}
