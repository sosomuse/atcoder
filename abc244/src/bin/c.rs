use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut vec: Vec<usize> = vec![];

    for i in 1..=n {
        vec.push(i);
    }

    while vec.len() > 0 {
        println!("{}", vec[0]);

        input! {
            o: usize,
        };

        vec.remove(o);
    }
}
