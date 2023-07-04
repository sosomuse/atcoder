use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    };

    let s = vec![0, 1, 2];

    if x == y {
        println!("{}", x);
    } else {
        for i in s {
            if i != x && i != y {
                println!("{}", i);
            }
        }
    }
}
