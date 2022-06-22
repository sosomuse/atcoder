use proconio::input;

fn main() {
    input! {
        (ax, ay): (f64, f64),
        (bx, by): (f64, f64),
        (cx, cy): (f64, f64),
    }

    let bax = ax - bx;
    let bay = ay - by;
    let bcx = cx - bx;
    let bcy = cy - by;
    let cax = ax - cx;
    let cay = ay - cy;
    let cbx = bx - cx;
    let cby = by - cy;

    let mut pt = 2;
    if (bax * bcx + bay * bcy) < 1e-6 {
        pt = 1;
    }
    if (cax * cbx + cay * cby) < 1e-6 {
        pt = 3;
    }

    match pt {
        1 => println!("{}", (bax * bax + bay * bay).sqrt()),
        3 => println!("{}", (cax * cax + cay * cay).sqrt()),
        2 => {
            let s = (bax * cay - bay * cax).abs();
            let bc_len = (bcx * bcx + bcy * bcy).sqrt();
            println!("{}", s / bc_len);
        }
        _ => println!("{}", "NO"),
    }
}
