use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };

    a.sort_by(|x, y| y.cmp(x));

    let mut odd = vec![];
    let mut even = vec![];

    for i in 0..n {
        let v = a[i];

        if v % 2 == 0 {
            even.push(v);
        } else {
            odd.push(v);
        }
    }

    if even.len() < 2 && odd.len() < 2 {
        println!("-1");
        return;
    }

    let even_max = {
        if even.len() >= 2 {
            even[0] + even[1]
        } else {
            0
        }
    };

    let odd_max = {
        if odd.len() >= 2 {
            odd[0] + odd[1]
        } else {
            0
        }
    };

    println!("{}", even_max.max(odd_max));
}
