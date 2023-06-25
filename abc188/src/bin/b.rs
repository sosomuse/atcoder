use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
        b: [isize; n],
    }

    let dot_product: isize = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();

    if dot_product == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
