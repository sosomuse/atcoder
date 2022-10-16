use proconio::input;

fn main() {
    input! {
        mut x: usize,
        k: usize,
    };

    // let len = x.to_string().len();

    for i in 0..k {
        let y = 10_usize.pow((i + 1) as u32);

        let a = x % y;
        let b = x - a;

        if a >= 5 * 10_usize.pow((i) as u32) {
            x = b + y;
        } else {
            x = b;
        }
    }

    println!("{}", x);
}
