use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
    };

    let mut set = std::collections::HashSet::new();

    set.insert(a);
    set.insert(b);
    set.insert(c);
    set.insert(d);
    set.insert(e);

    println!("{}", set.len());
}
