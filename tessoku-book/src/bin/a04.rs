use proconio::input;

fn main() {
    input! {
        n: u64,
    };

    println!("{:010}", convert_string(n, 2).parse::<u64>().unwrap());
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
