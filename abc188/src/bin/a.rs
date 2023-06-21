use proconio::input;

fn main() {
    input! {
        mut x: usize,
        mut y: usize,
    };

    if x > y {
        std::mem::swap(&mut x, &mut y);
    }

    if x + 3 > y {
        println!("Yes");
    } else {
        println!("No");
    }
}
