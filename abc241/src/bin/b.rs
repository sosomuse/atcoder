use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    };

    let mut ans = true;

    a.sort();
    b.sort();

    for i in 0..m {
        let x = b[i];
        let y = a.binary_search(&x);

        match y {
            Ok(y) => {
                a.remove(y);
            }
            Err(_) => {
                ans = false;
                break;
            }
        }
    }

    println!("{}", if ans { "Yes" } else { "No" });
}
