use proconio::input;

fn main() {
    input! {
        mut n: usize,
    };

    let mut ans = vec![];

    while n > 0 {
        if n > 26 {
            let m = n % 26;
            if m == 0 {
                ans.push('z');
                n = n / 26 - 1;
            } else {
                ans.push((m + 96) as u8 as char);
                n = n / 26;
            }
        } else {
            ans.push((n + 96) as u8 as char);
            n = 0;
        }
    }

    println!("{}", ans.iter().rev().collect::<String>());
}
