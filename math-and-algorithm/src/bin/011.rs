use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    for i in 2..=n {
        if solve(i) {
            print!("{} ", i);
        }
    }
}

fn solve(n: usize) -> bool {
    let mut is_prime = true;

    for i in 2..n {
        if n % i == 0 {
            is_prime = false;
            break;
        }
    }

    is_prime
}
