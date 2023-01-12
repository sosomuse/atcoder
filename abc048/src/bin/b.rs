use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        x: usize,
    };

    let mut q = b / x;
    let p = {
        if a == 0 {
            0
        } else {
            (a - 1) / x
        }
    };

    if a == 0 {
        q += 1;
    }

    println!("{}", q - p);
}
