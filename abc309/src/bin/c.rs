use proconio::input;

fn check(mid: i64, k: i64, medicine_info: &Vec<(i64, i64)>) -> bool {
    let mut total = 0i64;
    for &(a, b) in medicine_info {
        if mid <= a {
            total += b;
        }
    }
    total > k
}

fn main() {
    input! {
        n: usize,
        k: i64,
        medicine_info: [(i64, i64); n],
    }

    let mut ok = 0i64;
    let mut ng = 1e18 as i64;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if check(mid, k, &medicine_info) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok + 1);
}
