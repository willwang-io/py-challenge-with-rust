fn main() {
    let mut cur = String::from("1#");
    for _ in 0..30 {
        let m = cur.len();
        let mut i = 0;
        let b = cur.as_bytes();
        let mut next = String::new();
        for j in 0..m {
            if b[i] != b[j] {
                next = format!("{}{}{}", next, j - i, String::from(b[i] as char));
                i = j;
            }
        }
        cur = format!("{next}#");
    }
    println!("{}", cur.len() - 1);
}
