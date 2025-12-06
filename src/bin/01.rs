fn caesar_shift(c: char) -> char {
    if c.is_ascii_lowercase() {
        ((c as u8 - b'a' + 2) % 26 + b'a') as char
    } else {
        c
    }
}

fn translate(s: &str) -> String {
    s.chars().map(caesar_shift).collect()
}

fn main() {
    let msg = "g fmnc wms bgblr rpylqjyrc gr zw fylb. rfyrq ufyr amknsrcpq ypc dmp. bmgle gr gl zw fylb gq glcddgagclr ylb rfyr'q ufw rfgq rcvr gq qm jmle. sqgle qrpgle.kyicrpylq() gq pcamkkclbcb. lmu ynnjw ml rfc spj.";
    let decoded = translate(msg);
    println!("{}", decoded);

    let next_path = translate("map");
    println!("{}", next_path);
}
