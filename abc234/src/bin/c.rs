use proconio::input;

fn main() {
    input! {
        mut k: usize,
    };

    let mut vec: Vec<usize> = vec![];

    while k != 0 {
        let x = k % 2;

        let v = if x == 1 { 2 } else { 0 };
        vec.push(v);

        k /= 2;
    }

    for i in (0..vec.len()).rev() {
        print!("{}", vec[i]);
    }
}
