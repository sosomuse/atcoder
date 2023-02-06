use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [String; n]
    };

    let mut ans = vec![];

    for i in 0..k {
        ans.push(s[i].to_string());
    }

    ans.sort();

    for i in 0..ans.len() {
        println!("{}", ans[i]);
    }
}
