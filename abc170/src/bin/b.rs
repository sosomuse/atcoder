use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    };

    if x * 4 < y {
        println!("No");
        return;
    }

    if x * 2 > y {
        println!("No");
        return;
    }

    if y % 2 == 1 {
        println!("No");
        return;
    }

    println!("Yes");
}
