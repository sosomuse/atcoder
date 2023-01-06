use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    };

    let count = a.iter().filter(|&&x| x < 0).count();
    let b = a.iter().map(|&x| x.abs()).collect::<Vec<_>>();
    let sum = b.iter().sum::<isize>();
    let min = b.iter().min().unwrap();

    let ans = sum;

    if count % 2 == 0 {
        println!("{}", ans);
    } else {
        println!("{}", ans - 2 * min);
    }
}
