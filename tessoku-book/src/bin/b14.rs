use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    let mut x1 = vec![];
    let mut x2 = vec![];

    let mid = n / 2;
    let mid2 = n - mid;

    for bit in 0..(1 << mid) {
        let mut sum = 0;
        for i in 0..mid {
            if bit & (1 << i) != 0 {
                sum += a[i];
            }
        }
        x1.push(sum);
    }

    for bit in 0..(1 << mid2) {
        let mut sum = 0;
        for i in 0..mid2 {
            if bit & (1 << i) != 0 {
                sum += a[mid + i];
            }
        }
        x2.push(sum);
    }

    x1.sort();
    x2.sort();

    if let Ok(_) = x1.binary_search(&k) {
        println!("Yes");
        return;
    }

    if let Ok(_) = x2.binary_search(&k) {
        println!("Yes");
        return;
    }

    for i in 0..x1.len() {
        let v = x1[i];
        if let Ok(_) = x2.binary_search(&(k - v)) {
            println!("Yes");
            return;
        }
    }

    for i in 0..x2.len() {
        let v = x2[i];
        if let Ok(_) = x1.binary_search(&(k - v)) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
