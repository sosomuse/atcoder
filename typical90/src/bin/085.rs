use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        k: usize,
    };

    let mut divs = divisors(k);
    let len = divs.len();
    let mut ans = 0;
    let mut set = HashSet::new();

    divs.sort();

    for i in 0..len {
        set.insert(divs[i]);
    }

    for i in 0..len {
        let x = divs[i];
        if x * x > k {
            break;
        }
        if x * x * x > k {
            break;
        }

        for j in i..len {
            let y = divs[j];
            if x * y > k {
                break;
            }
            if x * y * y > k {
                break;
            }

            if k % (x * y) != 0 {
                continue;
            }

            if let Some(_) = set.get(&(k / (x * y))) {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}

fn divisors(n: usize) -> Vec<usize> {
    let mut lst: Vec<usize> = vec![];

    let mut i = 1;

    while i * i <= n {
        if n % i == 0 {
            lst.push(i);
            if i != n / i {
                lst.push(n / i);
            }
        }

        i += 1;
    }

    lst
}
