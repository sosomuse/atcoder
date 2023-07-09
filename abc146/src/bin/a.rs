use proconio::input;

fn main() {
    input! {
        s: String,
    };

    let days = vec!["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];
    let mut ans = 0;

    for i in 0..days.len() {
        if s == days[i] {
            ans = 7 - i;
        }
    }

    println!("{}", ans);
}
