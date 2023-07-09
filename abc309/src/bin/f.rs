use proconio::input;

fn main() {
    input! {
        n: usize,
        // 高さ・幅・奥行き
        boxes: [(usize, usize, usize); n],
    }

    // 高さ・幅・奥行きの順でソート
    let mut boxes_1 = boxes.clone();
    boxes_1.sort_by(|a, b| match a.0.cmp(&b.0) {
        std::cmp::Ordering::Equal => match a.1.cmp(&b.1) {
            std::cmp::Ordering::Equal => a.2.cmp(&b.2),
            default => default,
        },
        default => default,
    });
    // 高さ・奥行き・幅の順でソート
    let mut boxes_2 = boxes.clone();
    boxes_2.sort_by(|a, b| match a.0.cmp(&b.0) {
        std::cmp::Ordering::Equal => match a.2.cmp(&b.2) {
            std::cmp::Ordering::Equal => a.1.cmp(&b.1),
            default => default,
        },
        default => default,
    });
    // 幅・高さ・奥行きの順でソート
    let mut boxes_3 = boxes.clone();
    boxes_3.sort_by(|a, b| match a.1.cmp(&b.1) {
        std::cmp::Ordering::Equal => match a.0.cmp(&b.0) {
            std::cmp::Ordering::Equal => a.2.cmp(&b.2),
            default => default,
        },
        default => default,
    });
    // 幅・奥行き・高さの順でソート
    let mut boxes_4 = boxes.clone();
    boxes_4.sort_by(|a, b| match a.1.cmp(&b.1) {
        std::cmp::Ordering::Equal => match a.2.cmp(&b.2) {
            std::cmp::Ordering::Equal => a.0.cmp(&b.0),
            default => default,
        },
        default => default,
    });
    // 奥行き・高さ・幅の順でソート
    let mut boxes_5 = boxes.clone();
    boxes_5.sort_by(|a, b| match a.2.cmp(&b.2) {
        std::cmp::Ordering::Equal => match a.0.cmp(&b.0) {
            std::cmp::Ordering::Equal => a.1.cmp(&b.1),
            default => default,
        },
        default => default,
    });
    // 奥行き・幅・高さの順でソート
    let mut boxes_6 = boxes.clone();
    boxes_6.sort_by(|a, b| match a.2.cmp(&b.2) {
        std::cmp::Ordering::Equal => match a.1.cmp(&b.1) {
            std::cmp::Ordering::Equal => a.0.cmp(&b.0),
            default => default,
        },
        default => default,
    });

    // 6つのBoxからそれぞれ上回るBoxがあるか判定
    let mut ans = false;

    let check = |b: (usize, usize, usize), vec: &Vec<(usize, usize, usize)>| {
        let mut ok = 0;
        let mut ng = n;
        let b1 = b.0;
        let b2 = b.1;
        let b3 = b.2;

        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            let m1 = vec[mid].0;

            if b1 < m1 {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ng = n;

        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            let m2 = vec[mid].1;

            if b2 < m2 {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ng = n;

        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            let m3 = vec[mid].2;

            if b3 < m3 {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        if vec[ok].0 > b1 && vec[ok].1 > b2 && vec[ok].2 > b3 {
            true
        } else {
            false
        }
    };

    // 順番に2分探索
    for i in 0..n {
        let t = boxes[i];
        if check((t.0, t.1, t.2), &boxes_1)
            || check((t.0, t.2, t.1), &boxes_2)
            || check((t.1, t.0, t.2), &boxes_3)
            || check((t.1, t.2, t.0), &boxes_4)
            || check((t.2, t.0, t.1), &boxes_5)
            || check((t.2, t.1, t.0), &boxes_6)
        {
            ans = true;
            break;
        }
    }

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
