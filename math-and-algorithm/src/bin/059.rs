use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let x = n % 4;

    match x {
        0 => {
            println!("{}", 6);
        }
        1 => {
            println!("{}", 2);
        }
        2 => {
            println!("{}", 4);
        }
        3 => {
            println!("{}", 8);
        }
        _ => unreachable!(),
    }
}
