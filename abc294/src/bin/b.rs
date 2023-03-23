use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[u8; w]; h],
    };

    for i in 0..h {
        for j in 0..w {
            let v = a[i][j];
            if v == 0 {
                print!(".")
            } else {
                print!("{}", (b'A' + v - 1) as char)
            }
        }

        println!()
    }
}
