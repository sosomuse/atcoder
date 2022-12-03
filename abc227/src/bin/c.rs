use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut ans = 0;
    ans += n;
    let mut t = n / 2;
    let mut wari = 2;

    while t >= n / t {
        ans += fnc(n).len() / 2;
        wari += 1;
        t = n / wari;
    }

    println!("{}", ans);
}

fn fnc(n: usize) -> Vec<usize> {
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
