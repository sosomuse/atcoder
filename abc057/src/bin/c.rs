use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut divisor = divisors(n);
    divisor.sort();
    println!("{}", divisor[divisor.len() / 2].to_string().len());
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
