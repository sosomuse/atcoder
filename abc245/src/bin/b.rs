use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
    }

    a.sort();

    let mut ans: isize = -1;

    for v in a {
        if v == ans || v == ans + 1 {
            ans = v;
        } else {
            println!("{}", ans + 1);
            return;
        }
    }

    println!("{}", ans + 1)
}
