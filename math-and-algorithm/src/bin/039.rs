use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut vec: Vec<isize> = vec![0; n + 1];

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
            x: isize,
        }

        vec[l - 1] += x;
        vec[r] -= x;
    }

    for i in 1..n {
        let v = vec[i];

        if v > 0 {
            print!("<");
        } else if v == 0 {
            print!("=");
        } else if v < 0 {
            print!(">");
        }
    }
}
