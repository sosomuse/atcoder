use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    };

    let mut set = std::collections::HashSet::new();

    for i in 0..n {
        let v = format!("{} {}", &st[i].0, &st[i].1);

        if set.contains(&v) {
            println!("Yes");
            return;
        }

        set.insert(v);
    }

    println!("No");
}
