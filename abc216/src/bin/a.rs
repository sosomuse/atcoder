use proconio::input;

fn main() {
    input! {
        s: String,
    };

    let a = s.split('.').collect::<Vec<_>>();
    let x = a[0];
    let y: usize = a[1].parse().unwrap();

    if y <= 2 {
        println!("{}-", x);
    } else if y <= 6 {
        println!("{}", x);
    } else {
        println!("{}+", x);
    }
}
