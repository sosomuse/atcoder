use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut ans = 0;

    for c in s.chars().into_iter() {
        if c == '1' {
            ans += 1;
        }
    }



    println!("{}", ans);
}
