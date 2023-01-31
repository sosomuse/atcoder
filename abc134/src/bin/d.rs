use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut b = vec![0; n];

    for i in (1..=n).rev() {
        let mut x = 0;
        let mut j = i * 2;
        while j <= n {
            if b[j - 1] == 1 {
                x += 1;
            }
            j += i;
        }
        if x % 2 != a[i - 1] {
            b[i - 1] = 1;
        }
    }

    let mut ans = vec![];
    for i in 0..n {
        if b[i] == 1 {
            ans.push(i + 1);
        }
    }

    println!("{}", ans.len());
    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }
}
