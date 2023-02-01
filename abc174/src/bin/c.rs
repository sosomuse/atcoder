use proconio::input;

fn main() {
    input! {
        k: usize,
    };

    let mut a = vec![0; k + 1];

    for i in 0..=k {
        if i == 0 {
            a[i] = 7 % k;
        } else {
            a[i] = (a[i - 1] * 10 + 7) % k;
        }

        if a[i] == 0 {
            println!("{}", i + 1);
            return;
        }
    }

    println!("-1");
}
