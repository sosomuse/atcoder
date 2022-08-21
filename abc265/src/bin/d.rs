use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        a: [usize; n],
    };

    let mut sums = vec![0; n + 1];

    for i in 1..=n {
        sums[i] = a[i - 1];
        if i > 1 {
            sums[i] += sums[i - 1];
        }
    }

    for x in 0..n {
        let v = sums[x];
        if let Ok(y) = sums.binary_search(&(v + p)) {
            if let Ok(z) = sums.binary_search(&(sums[y] + q)) {
                if let Ok(_) = sums.binary_search(&(sums[z] + r)) {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
