fn main() {
    // 最大公約数
    let a = gcd(4, 6);
    println!("{}", a);

    // 最小公倍数
    let a = icm(4, 6);
    println!("{}", a);

    // 総和
    let a = sum(4);
    println!("{}", a);

    // 二分探索
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let a = binary_search(&vec, 6);
    println!("{}", a);

    let a = sigma(1, 5);
    println!("{}", a);

    // exit
    std::process::exit(0);
}

// ユークリッドの互除法（最大公約数）
fn gcd(mut a: usize, mut b: usize) -> usize {
    let mut r = a % b;
    while r != 0 {
        a = b;
        b = r;
        r = a % b;
    }

    return b;
}

// 最小公倍数
fn icm(a: usize, b: usize) -> usize {
    return a * b / gcd(a, b);
}

// 総和
fn sum(a: usize) -> usize {
    (a + 1) * (a / 2)
}

// 二分探索
fn binary_search(a: &Vec<i32>, key: i32) -> usize {
    let mut l = 0;
    let mut r = a.len() - 1;
    while l <= r {
        let m = (l + r) / 2;
        if a[m] == key {
            return m;
        } else if a[m] < key {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }

    return l;
}

// sigma
fn sigma(l: usize, r: usize) -> usize {
    return (r - l + 1) * (r + l) / 2;
}
