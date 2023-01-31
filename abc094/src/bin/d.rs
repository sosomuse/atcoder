use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
    };

    a.sort();

    let max = a[n - 1];
    let mut mid = a[0];

    for i in 1..n - 1 {
        if (max / 2 - a[i]).abs() <= (max / 2 - mid).abs() {
            mid = a[i];
        }
    }

    println!("{} {}", max, mid);
}
