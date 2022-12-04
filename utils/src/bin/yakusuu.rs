fn main() {
    let a = fnc(280);
    println!("{:?}", a);
}

// 約数の列挙
fn fnc(n: usize) -> Vec<usize> {
    let mut lst: Vec<usize> = vec![];

    let mut i = 1;

    while i * i <= n {
        if n % i == 0 {
            lst.push(i);
            if i != n / i {
                lst.push(n / i);
            }
        }

        i += 1;
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
