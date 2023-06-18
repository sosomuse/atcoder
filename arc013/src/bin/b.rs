use proconio::input;

fn main() {
    input! {
        c: usize,
        mut nml: [[usize; 3]; c],
    };

    for i in 0..c {
        nml[i].sort();
    }

    let mut n = 0;
    let mut m = 0;
    let mut l = 0;

    for i in 0..c {
        n = n.max(nml[i][0]);
        m = m.max(nml[i][1]);
        l = l.max(nml[i][2]);
    }

    println!("{}", n * m * l);
}
