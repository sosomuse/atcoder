use proconio::input;

fn main() {
    input! {
        s: [String; 4],
    };

    let mut set = std::collections::HashSet::new();
    for i in 0..4 {
        set.insert(s[i].clone());
    }

    if set.len() == 4 {
        println!("Yes");
    } else {
        println!("No");
    }
}
