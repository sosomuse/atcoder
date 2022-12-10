use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: [usize; n],
    };

    let sum = a.iter().sum::<usize>();
    let mut m = t % sum;

    for i in 0..n {
        let v = a[i];

        if m > v {
            m -= v;
        } else {
            println!("{} {}", i + 1, m);
            return;
        }
    }
}
