use proconio::input;

fn main() {
    input! {
        s: String,
    };

    match s.as_str() {
        "Sunny" => println!("Cloudy"),
        "Cloudy" => println!("Rainy"),
        "Rainy" => println!("Sunny"),
        _ => unreachable!(),
    }
}
