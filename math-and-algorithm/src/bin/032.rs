use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        mut a: [usize; n],
    }

    a.sort();
    let ans = a.binary_search(&x);

    match ans {
        Ok(_) => println!("Yes"),
        Err(_) => println!("No"),
    }
}
