use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    solve(n);
}

fn solve(mut n: usize) -> () {
    let mut i = 2;

    while i * i <= n {
        if n % i == 0 {
            print!("{} ", i);
            n /= i;
        } else {
            i += 1;
        }
    }

    print!("{}", n);
}
