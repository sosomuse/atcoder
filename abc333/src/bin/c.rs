use proconio::input;

fn main() {
    input! {
        mut n: usize,
    };

    let r: Vec<usize> = vec![
        1,
        11,
        111,
        1111,
        11111,
        111111,
        1111111,
        11111111,
        111111111,
        1111111111,
        11111111111,
        111111111111,
    ];

    for i in 0..12 {
        for j in 0..i + 1 {
            for k in 0..j + 1 {
                if n - 1 == 0 {
                    println!("{}", r[i] + r[j] + r[k]);
                    return;
                }
                n -= 1;
            }
        }
    }
}
