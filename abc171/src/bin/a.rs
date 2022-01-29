use proconio::input;
fn main() {
    input! {
        c: char,
    }
    let ans = match c {
        'A'..='Z' => 'A',
        'a'..='z' => 'a',
        _ => unreachable!(),
    };
    println!("{}", ans);
}
