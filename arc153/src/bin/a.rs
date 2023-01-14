use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let a = 100000;
    let r = 110000000;
    let t = 10000000;

    let mut p = n / a + 1;
    let mut q = n % a;
    let mut count = 0;

    if q == 0 {
        p -= 1;
        q = a;
    }

    for i in r * p..r * p + t {
        let i9 = i % 10;
        let i7 = i % 1000 / 100;
        let i6 = i % 10000 / 1000;
        let i5 = i % 100000 / 10000;
        let i2 = i % 100000000 / 10000000;

        if p == i2 && i5 == i6 && i7 == i9 {
            count += 1;
        }

        if count == q {
            println!("{}", i);
            return;
        }
    }
}
