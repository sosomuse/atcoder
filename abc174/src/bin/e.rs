use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    let mut low = 1;
    let mut high = *a.iter().max().unwrap();

    while low != high {
        let mid = (low + high) / 2;
        let mut count = 0;

        for i in 0..n {
            if a[i] > mid {
                count += a[i] / mid;
            }
        }

        if count <= k {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    println!("{}", low);
}
