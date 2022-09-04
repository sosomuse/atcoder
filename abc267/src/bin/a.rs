use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let vec: Vec<String> = vec![
        "Monday".to_string(),
        "Tuesday".to_string(),
        "Wednesday".to_string(),
        "Thursday".to_string(),
        "Friday".to_string(),
    ];

    for i in 0..5 {
        if s == vec[i] {
            println!("{}", 5 - i);
        }
    }
}
