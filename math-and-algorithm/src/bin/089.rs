use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };

    if c <= 1 {
        println!("No");
        return;
    }

    let ans = solve(a) < b || solve(a) < b * solve(c);

    println!("{}", if ans { "Yes" } else { "No" });
}

fn solve(mut n: usize) -> usize {
    let mut count = 0;

    while n > 1 {
        n /= 2;
        count += 1;
    }

    count
}
