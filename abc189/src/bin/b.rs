use proconio::input;

fn main() {
    input! {
        (n, x): (u32, u32),
        a: [(u32, u32); n],
    }

    let mut ans: i32 = -1;
    let mut al: u32 = 0;

    for (i, (v, p)) in a.iter().enumerate() {
        al += v * p;
        if x * 100 < al {
            ans = (i + 1) as i32;
            break;
        }
    }

    println!("{}", ans)
}
