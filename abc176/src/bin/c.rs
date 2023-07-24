use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };

    let mut sum = 0;

    for i in 1..n {
        let x = a[i - 1];
        let y = a[i];

        if x > y {
            sum += x - y;
            a[i] = x;
        }
    }

    println!("{}", sum);
}
