use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
    };

    loop {
        if a <= 0 || b <= 0 {
            println!("Easy");
            return;
        }

        let x = a % 10;
        let y = b % 10;

        if x + y >= 10 {
            println!("Hard");
            return;
        }

        a /= 10;
        b /= 10;
    }
}
