use proconio::input;

fn main() {
    input! {
        mut s: String,
    }

    let len = s.len();
    let l = 6 / len + 1;
    let mut ans = "".to_string();

    for _ in 0..l {
        ans += &s;
    }

    print!("{}", &ans[0..6]);
}
