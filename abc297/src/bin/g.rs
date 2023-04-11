use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        mut a: [usize; n],
    }

    let sum = l + r;
    let mut count = 0;

    for i in 0..n {
        if a[i] < l {
            continue;
        }

        if a[i] > 0 {
            count += a[i] / sum * 2;
            a[i] %= sum;
        }

        if a[i] > r {
            count += 2;
        } else if a[i] <= r {
            if a[i] < l {
                continue;
            }

            count += 1;
        }
    }

    if count % 2 == 1 {
        println!("First");
    } else {
        println!("Second");
    }
}
