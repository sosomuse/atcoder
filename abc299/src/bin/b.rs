use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        c: [usize; n],
        r: [usize; n],
    }

    let mut max_t_value = 0;
    let mut max_t_player = 0;
    let mut max_same_value = 0;
    let mut max_same_player = 0;

    for i in 0..n {
        let cv = c[i];
        let rv = r[i];

        if cv == t && rv > max_t_value {
            max_t_value = rv;
            max_t_player = i + 1;
        } else if i == 0 || (cv == c[0] && rv > max_same_value) {
            max_same_value = rv;
            max_same_player = i + 1;
        }
    }

    if max_t_value > 0 {
        println!("{}", max_t_player);
    } else {
        println!("{}", max_same_player);
    }
}
