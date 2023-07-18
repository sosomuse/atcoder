use proconio::input;

fn main() {
    input! {
        a: [usize; 4],
    };

    println!("{}", a.iter().min().unwrap());
}
