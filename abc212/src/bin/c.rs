use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    };

    a.sort();
    b.sort();

    let mut ans = 1000000000;

    for i in 0..n {
        let v = a[i];

        let j = b.binary_search(&v).unwrap_or_else(|j| j) as isize;

        for k in j - 1..j + 1 {
            if k < 0 || k >= m as isize {
                continue;
            }

            let w = b[k as usize];
            ans = ans.min((v as isize - w as isize).abs() as usize);
        }
    }

    println!("{}", ans);
}
