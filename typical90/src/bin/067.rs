use proconio::input;

fn main() {
    input! {
        n: String,
        k: usize,
    };

    let mut ans = String::from(n);

    for _ in 0..k {
        let x = u64::from_str_radix(&ans, 8).unwrap();
        let y = convert_string(x, 9);
        ans = y.replace('8', "5");
    }

    println!("{}", ans);
}

fn convert_string(mut n: u64, base: u64) -> String {
    let mut ans = vec![];
    loop {
        let a = n % base;
        let b = n / base;

        ans.push(format!("{}", a));
        if b == 0 {
            break;
        }
        n = b;
    }

    ans.reverse();
    ans.join("")
}
