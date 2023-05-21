use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut day = 0;
    let mut money = 0;

    while money < n {
        day += 1;
        money += day;
    }

    println!("{}", day);
}
