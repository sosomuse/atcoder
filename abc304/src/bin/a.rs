use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        people: [(Chars, usize); n],
    }

    let min_pos = people.iter().min_by_key(|&x| x.1).unwrap().1;
    let pos = people.iter().position(|x| x.1 == min_pos).unwrap();

    for i in pos..n {
        println!("{}", people[i].0.iter().collect::<String>());
    }

    for i in 0..pos {
        println!("{}", people[i].0.iter().collect::<String>());
    }
}
