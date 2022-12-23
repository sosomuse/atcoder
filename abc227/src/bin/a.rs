use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: usize,
    };

    let ans = k % a;

    if ans == 0 {
        println!("{}", n)
    } else {
        println!("{}", ans)
    }
}
