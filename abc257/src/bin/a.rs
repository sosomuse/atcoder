use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
    }

    let mut s = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    s = s.repeat(n);

    let mut it: Vec<char> = s.chars().collect();
    it.sort();

    println!("{}", it[x - 1]);
}
