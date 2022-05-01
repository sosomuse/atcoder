use proconio::input;

fn main() {
    input! {
        a: [usize; 10],
    }

    let mut i = 0;

    for _ in 0..3 {
        i = a[i];
    }

    println!("{}", i);
}
