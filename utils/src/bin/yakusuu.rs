fn main() {
    let a = fnc(100);
    println!("{:?}", a);

    let a = fnc_2(4);
    println!("{:?}", a);
}

fn fnc(n: usize) -> Vec<usize> {
    let mut lst: Vec<usize> = vec![];

    let mut count = 1;

    while count * count <= n {
        if n % count == 0 {
            lst.push(count);
            if count != n / count {
                lst.push(n / count);
            }
        }

        count += 1;
    }

    lst
}

// 約数の個数を求める
fn fnc_2(n: usize) -> Vec<usize> {
    let mut num_factors = vec![1; n + 1];
    for i in 2..=n {
        let mut j = i;
        while j <= n {
            num_factors[j] += 1;
            j += i;
        }
    }

    num_factors
}
