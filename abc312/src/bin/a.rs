use proconio::input;

fn main() {
    input! {
        s: String,
    };

    let vec = ["ACE", "BDF", "CEG", "DFA", "EGB", "FAC", "GBD"];

    if vec.contains(&s.as_str()) {
        println!("Yes");
    } else {
        println!("No");
    }
}
