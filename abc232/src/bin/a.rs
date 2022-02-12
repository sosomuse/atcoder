use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for (i, c) in s.chars().enumerate() {
        if i == 0 {
            x = c.to_string().parse().unwrap();
        } else if i == 2 {
            y = c.to_string().parse().unwrap();
        }
    }

    println!("{}", x * y)
}
