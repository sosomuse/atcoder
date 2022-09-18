use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
        d: [usize; n],
    };

    let mut ab = vec![];
    let mut cd = vec![];

    for i in 0..n {
        for j in 0..n {
            ab.push(a[i] + b[j]);
            cd.push(c[i] + d[j]);
        }
    }

    ab.sort();
    cd.sort();

    for i in 0..ab.len() {
        let v = ab[i];
        if let Ok(_) = cd.binary_search(&(k - v)) {
            println!("Yes");
            return;
        }
    }

    for i in 0..cd.len() {
        let v = cd[i];
        if let Ok(_) = ab.binary_search(&(k - v)) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
