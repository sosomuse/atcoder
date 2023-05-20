use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
        b: [isize; n],
    };

    let mut count = 0isize;

    for i in 0..n {
        if a[i] < b[i] {
            count += (b[i] - a[i]) / 2;
        } else {
            count -= a[i] - b[i];
        }
    }

    if count >= 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
