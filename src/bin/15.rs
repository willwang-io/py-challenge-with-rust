fn find_day(y: usize, m: usize, d: usize) -> usize {
    let mut m = m;
    let mut y = y;
    if m < 3 {
        m += 12;
        y -= 1;
    }
    // 0 is Saturday
    (d + (13 * (m + 1)) / 5 + y + y / 4 - y / 100 + y / 400) % 7
}

fn main() {
    for y in 1500..2000 {
        if y % 10 == 6 && find_day(y, 1, 26) == 2 {
            println!("{y}");
        }
    }
}
