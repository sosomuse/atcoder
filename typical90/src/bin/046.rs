use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    };

    let mut a_count = vec![0; 46];
    let mut b_count = vec![0; 46];
    let mut c_count = vec![0; 46];

    for i in 0..n {
        a_count[a[i] % 46] += 1;
        b_count[b[i] % 46] += 1;
        c_count[c[i] % 46] += 1;
    }

    let mut ans: usize = 0;

    for i in 0..46 {
        for j in 0..46 {
            for k in 0..46 {
                if (i + j + k) % 46 == 0 {
                    ans += a_count[i] * b_count[j] * c_count[k];
                }
            }
        }
    }

    println!("{}", ans);
}
