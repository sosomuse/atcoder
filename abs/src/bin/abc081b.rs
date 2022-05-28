use proconio::input;

fn main() {
    input! {
        s: u32,
        mut a: [u64; s],
    }

    let mut ans = 0;

    for _ in 0..200 {
        let is_even = a.iter().all(|&x| x % 2 == 0);

        if !is_even {
            break;
        }

        a = a.iter().map(|&x| x / 2).collect();
        ans += 1;
    }

    println!("{}", ans);
}
