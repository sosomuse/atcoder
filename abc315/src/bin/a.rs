use proconio::input;

fn main() {
    input! {
        s: String,
    };

    s.chars().for_each(|c| {
        if c != 'a' && c != 'i' && c != 'u' && c != 'e' && c != 'o' {
            print!("{}", c);
        }
    });
}
