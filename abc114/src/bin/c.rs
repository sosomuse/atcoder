use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut ans = 0;
    for i in 0..3 {
        let s = ["3", "5", "7"][i].to_string();
        solve(n, s, &mut ans);
    }

    println!("{}", ans);
}

fn solve(n: usize, s: String, ans: &mut usize) {
    let x: usize = s.parse().unwrap();
    if x > n {
        return;
    }

    if s.contains("3") && s.contains("5") && s.contains("7") {
        *ans += 1;
    }

    for i in 0..3 {
        let s = s.clone() + ["3", "5", "7"][i];
        solve(n, s, ans);
    }
}
