use proconio::input;

fn main() {
    input! {
        s: [String; 3],
    };

    let mut t = vec![
        "ABC".to_string(),
        "ARC".to_string(),
        "AGC".to_string(),
        "AHC".to_string(),
    ];

    for s_i in s {
        t.retain(|t_i| t_i != &s_i);
    }

    println!("{}", t[0]);
}
