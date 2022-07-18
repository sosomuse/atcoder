use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n * 4 - 1],
    }

    let mut vec = vec![0; n];

    for i in 0..a.len() {
        vec[a[i] - 1] += 1;
    }

    for i in 0..n {
        if vec[i] != 4 {
            println!("{}", i + 1);
            return;
        }
    }
}
