fn main() {
    let a = sigma(1, 5);
    println!("{}", a);

    let a = sigma2(2, 4, 3, 5);
    println!("{}", a);
}

fn sigma(i: i32, r: i32) -> i32 {
    let mut ans = 0;

    for v in i..=r {
        ans += v * v;
    }

    ans
}

fn sigma2(i1: i32, i2: i32, r1: i32, r2: i32) -> i32 {
    let mut ans = 0;

    for v in i1..=r1 {
        for w in i2..=r2 {
            ans += v + w;
        }
    }

    ans
}
