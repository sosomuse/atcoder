use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    if n <= 10u32.pow(3) - 1 {
        println!("{}", n);
        return;
    }

    if n <= 10u32.pow(4) - 1 {
        println!("{}", n - n % 10);
        return;
    }

    if n <= 10u32.pow(5) - 1 {
        println!("{}", n - n % 100);
        return;
    }

    if n <= 10u32.pow(6) - 1 {
        println!("{}", n - n % 1000);
        return;
    }

    if n <= 10u32.pow(7) - 1 {
        println!("{}", n - n % 10000);
        return;
    }

    if n <= 10u32.pow(8) - 1 {
        println!("{}", n - n % 100000);
        return;
    }

    if n <= 10u32.pow(9) - 1 {
        println!("{}", n - n % 1000000);
        return;
    }
}
