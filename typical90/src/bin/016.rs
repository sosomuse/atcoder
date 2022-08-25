use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: usize,
    };

    let mut ans = 10000000000000;

    for i in 0..=9999 {
        let x = i * a;

        if x > n {
            continue;
        }

        for j in 0..=9999 {
            let y = j * b;

            if x + y > n {
                continue;
            }

            let z = n - x - y;
            let k = z / c;

            if x + y + z == n && z % c == 0 {
                ans = ans.min(i + j + k);
            }
        }
    }

    println!("{}", ans);
}
