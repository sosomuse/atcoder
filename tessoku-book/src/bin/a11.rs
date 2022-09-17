use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    };

    println!("{}", binary_search(x, n, a) + 1);
}

fn binary_search(x: usize, mut r: usize, a: Vec<usize>) -> usize {
    let mut l = 0;

    while l <= r {
        let mid = (l + r) / 2;
        if a[mid] == x {
            return mid;
        } else if a[mid] < x {
            l = mid + 1;
        } else if a[mid] > x {
            r = mid - 1;
        }
    }

    return 0;
}
