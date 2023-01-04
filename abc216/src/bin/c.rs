use proconio::input;

fn main() {
    input! {
        mut n: usize,
    };

    let mut ans = String::from("");

    while n != 0 {
        if n % 2 == 1 {
            ans.push('A');
            n -= 1;
        } else {
            ans.push('B');
            n /= 2;
        }
    }

    println!("{}", ans.chars().rev().collect::<String>());
}
