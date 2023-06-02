use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [String; n],
    };

    for i in 0..n {
        s[i] = s[i].chars().rev().collect::<String>();
    }

    s.sort();

    for i in 0..n {
        println!("{}", s[i].chars().rev().collect::<String>());
    }
}
