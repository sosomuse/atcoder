use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut vec: Vec<char> = s.chars().collect();

    vec.sort();

    let ans: String = vec.into_iter().collect();

    println!("{}", ans);
}
