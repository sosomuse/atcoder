use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: usize,
        b: usize,
    };

    for i in 0..h {
        for j in 0..w {
            if i < b {
                if j < a {
                    print!("0");
                } else {
                    print!("1");
                }
            } else {
                if j < a {
                    print!("1");
                } else {
                    print!("0");
                }
            }
        }

        println!();
    }
}
