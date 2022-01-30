use proconio::input;

// 2, 20, 22, 200, 202, 220, 222, 2000, 2002, 2020, 2022, 2200, 2202, 2220, 2222
// 1, 2, 4, 8
fn main() {
    input! {
        k: u128,
    }

    let ans = format!("{:b}", k).to_string();

    for c in ans.chars() {
        match c {
            '1' => print!("2"),
            _ => print!("0"),
        }
    }
}
