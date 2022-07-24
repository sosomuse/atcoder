use proconio::input;

fn main() {
    input! {
        l1: usize,
        r1: usize,
        l2: usize,
        r2: usize,
    };

    if l1 <= l2 {
        if r1 <= l2 {
            println!("0");
            return;
        } else {
            println!("{}", (r1 - l2).min(r2 - l2));
        }
    } else {
        if l1 >= r2 {
            println!("0");
            return;
        } else {
            println!("{}", (r2 - l1).min(r1 - l1));
        }
    }
}
