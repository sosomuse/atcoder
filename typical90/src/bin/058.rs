use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        mut n: usize,
        mut k: usize,
    };

    let mut indexes = HashMap::new();
    let mut count = 0;
    let mut r = 0;

    for _ in 0..100000.min(k) {
        let next = {
            let mut c = n;
            let mut a = c;

            while c > 0 {
                let d = c % 10;
                c /= 10;

                a += d;
            }
            a = a % 100000;

            a
        };

        if let Some(&i) = indexes.get(&next) {
            count += 1;
            n = next;
            r = count - i;
            break;
        }

        indexes.insert(n, count);
        n = next;
        count += 1;
    }

    if r == 0 {
        println!("{}", n);
        return;
    }

    k -= count;
    k %= r;

    for _ in 0..k {
        let mut a = n;
        while n > 0 {
            let d = n % 10;
            n /= 10;
            a += d;
        }
        a = a % 100000;

        n = a;
    }

    println!("{}", n);
}
