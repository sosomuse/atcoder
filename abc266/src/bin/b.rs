use proconio::input;

fn main() {
    input! {
        n: i64,
    };

    if n % 998244353 == 0 {
        println!("0");
        return;
    }

    if n >= 998244353 {
        println!("{}", n % 998244353);
    } else {
        if n >= 0 {
            println!("{}", n);
            return;
        }

        let v = n % 998244353;

        if v >= 0 {
            println!("{}", 998244353 - v);
        } else {
            println!("{}", 998244353 + v);
        }
    }
}
