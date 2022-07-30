use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    };

    if y <= x {
        println!("0");
        return;
    }

    let mut ans = (y - x) / 10;

    if (y - x) % 10 != 0 {
        ans += 1;
    }

    println!("{}", ans);
}
