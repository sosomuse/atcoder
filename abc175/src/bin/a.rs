use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut count = 0;
    let mut ans = 0;

    for v in s.chars() {
        match v {
            'R' => count += 1,
            'S' => count = 0,
            _ => unreachable!(),
        }

        ans = ans.max(count);
    }

    println!("{}", ans);
}
