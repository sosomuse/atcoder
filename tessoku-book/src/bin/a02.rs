use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        mut a: [usize; n]
    };

    a.sort();

    if let Ok(_) = a.binary_search(&x) {
        println!("Yes");
    } else {
        println!("No");
    }
}
