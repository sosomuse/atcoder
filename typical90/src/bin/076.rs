use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let sum = a.iter().sum::<usize>();

    if sum % 10 != 0 {
        println!("No");
        return;
    }

    let mut vec = vec![0; n * 2 + 1];

    for i in 1..=n {
        let x = a[i - 1];
        vec[i] = x + vec[i - 1];
    }

    for i in n..n * 2 {
        let x = a[i - n];
        vec[i + 1] = x + vec[i];
    }

    for i in 0..=n {
        let x = vec[i];
        if let Ok(_) = vec.binary_search(&(x + sum / 10)) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
