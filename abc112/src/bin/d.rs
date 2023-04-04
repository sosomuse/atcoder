use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut ans = 1;
    let divs = divisors(m);

    for d in divs {
        if n <= m / d {
            ans = d;
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
