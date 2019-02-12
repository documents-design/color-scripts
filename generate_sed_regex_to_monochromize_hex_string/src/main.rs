use std::env;
use std::i64;

fn h_i(h2: &str) -> i64 {
    i64::from_str_radix(h2, 16).unwrap()
}

fn hex_to_rgba(color: &str) -> (i64, i64, i64, i64) {
    let l = color.to_string();
    let len = l.len();
    match len {
        // #01234567
        9 => (h_i(&l[1..3]), h_i(&l[3..5]), h_i(&l[5..7]), h_i(&l[7..])),
        // 01234567
        8 => (h_i(&l[0..2]), h_i(&l[2..4]), h_i(&l[4..6]), h_i(&l[6..])),
        // #012345
        7 => (h_i(&l[1..3]), h_i(&l[3..5]), h_i(&l[5..]), 255),
        // 012345
        6 => (h_i(&l[0..2]), h_i(&l[2..4]), h_i(&l[4..]), 255),
        // #FFF
        4 => (
            h_i(l[1..2].repeat(2).as_str()),
            h_i(l[2..3].repeat(2).as_str()),
            h_i(l[3..].repeat(2).as_str()),
            255
        ),
        // FFF
        3 => (
            h_i(l[0..1].repeat(2).as_str()),
            h_i(l[1..2].repeat(2).as_str()),
            h_i(l[2..].repeat(2).as_str()),
            255
        ),
        _ => (0, 0, 0, 0),
    }
}

fn rgba_to_hex(color: (i64, i64, i64, i64)) -> String {
    format!("{:2X}{:2X}{:2X}{:2X}", color.0, color.1, color.2, color.3).to_string()
}

fn average_rgb(color: (i64, i64, i64, i64)) -> (i64, i64, i64, i64) {
    let mut avg = (color.0 * 21 + color.1 * 71 + color.2 * 7) / 100;
    avg = if avg < 0 {
        0
    } else {
        if avg > 255 {
            255
        } else {
            avg
        }
    };
    (avg, avg, avg, color.3)
}

fn main() {
    let color = if env::args().count() >= 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("enter a color please");
    };

    let orig = env::args().count() > 2;

    if orig {
        println!(
            "'s/{}/{}/g'",
            color[1..].to_string(),
            rgba_to_hex(average_rgb(hex_to_rgba(color.as_str())))
        );
    } else {
        println!("{}", rgba_to_hex(average_rgb(hex_to_rgba(color.as_str()))));
    }
}
