fn main() {
    if is_prime(121) {
        println!("Yes");
    } else {
        println!("No");
    }
}

// O(√n)
fn is_prime(n: usize) -> bool {
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }

        i += 1;
    }

    return true;
}
