use ::proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    };

    let mut ans = 0;
    let mut vec: Vec<usize> = vec![];

    for i in l..=r {
        vec.push(i);
    }

    let mut i = 3;
    while i * i <= l {
        let len = vec.len();
        for (i, v) in vec.iter_mut().enumerate() {
            // let v = vec[0];
            if *v == 1 {
                ans += 1;
                continue;
            }
            if *v == 2 {
                vec.remove(i);
                ans += 1;
                continue;
            }
            if *v % i == 0 {
                vec.remove(i);
                continue;
            }
        }
        i += 2;
    }

    dbg!(&vec);

    println!("{}", ans);
}
