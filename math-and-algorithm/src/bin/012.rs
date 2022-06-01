use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    if solve(n) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn solve(n: usize) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }

        if i.pow(2) >= n {
            break;
        }
    }

    return true;
}
