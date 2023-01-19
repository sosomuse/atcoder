use proconio::input;

fn main() {
    input! {
        p: [usize; 26],
    };

    for i in 0..p.len() {
        let v = p[i];
        let n = b'a' + v as u8 - 1;

        print!("{}", n as char)
    }
}
