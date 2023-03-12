use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            mut x: [usize; 3],
        };

        x.sort();
        let mut x1 = x[0];
        let x2 = x[1];
        let x3 = x[2];

        if !is_even(x3, x2) || !is_even(x3, x1) {
            println!("-1");
            continue;
        }

        let count = ((x2 - x1) / 2).min((x3 - x2) / 2);
        x1 += count * 4;

        if (x3 - x1) % 6 == 0 {
            println!("{}", (x3 - x1) / 6 * 2 + count);
        } else {
            println!("-1");
            continue;
        }
    }
}

// aとbで引いた時の答えが偶数かどうか
fn is_even(a: usize, b: usize) -> bool {
    (a - b) % 2 == 0
}
