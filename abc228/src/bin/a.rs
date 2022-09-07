use proconio::input;

fn main() {
    input! {
        s: usize,
        mut t: usize,
        x: usize,
    };

    let mut ans = false;

    if s > t {
        if x < t {
            ans = true;
        }

        t += 24;
    }
    if s <= x && x < t {
        ans = true;
    }

    println!("{}", if ans { "Yes" } else { "No" });
}
