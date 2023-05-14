use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    };

    let t_count = s.iter().filter(|&&c| c == 'T').count();
    let a_count = s.iter().filter(|&&c| c == 'A').count();
    let t_last = s.iter().rposition(|&c| c == 'T').unwrap_or(0);
    let a_last = s.iter().rposition(|&c| c == 'A').unwrap_or(0);

    if t_count > a_count {
        println!("T");
        return;
    }

    if t_count < a_count {
        println!("A");
        return;
    }

    if t_last < a_last {
        println!("T");
        return;
    }

    println!("A");
    return;
}
