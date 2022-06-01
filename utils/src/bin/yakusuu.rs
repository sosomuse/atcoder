fn main() {
    // ステップ数の指定
    let a = fnc(100);
    println!("{:?}", a);
}

fn fnc(n: usize) -> Vec<usize> {
    let mut vec = Vec::new();
    for i in 1..=n {
        if i.pow(2) >= n {
            break;
        }

        if n % i != 0 {
            continue;
        }

        vec.push(i);

        if i != n / i {
            vec.push(n / i);
        }
    }
    return vec;
}
