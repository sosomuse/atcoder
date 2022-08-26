use proconio::input;

fn main() {
    input! {
        n: usize,
        k: isize,
        a: [isize; n],
        b: [isize; n],
    };

    let mut count = 0;

    for i in 0..n {
        count += (a[i] - b[i]).abs();
    }

    if k >= count && (k - count) % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
