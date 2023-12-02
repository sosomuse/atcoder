use proconio::input;

fn main() {
    input! {
        b: usize,
    };

    for i in 1..20 {
        let (x, is_over) = (i as usize).overflowing_pow(i);
        if is_over {
            println!("-1");
            return;
        }

        if x == b {
            println!("{}", i);
            return;
        }
    }
}
