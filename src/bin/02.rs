use regex::Regex;
use std::collections::HashMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.pythonchallenge.com/pc/def/ocr.html";
    let body = reqwest::blocking::get(url)?.text()?;

    let re_comments = Regex::new(r"(?s)<!--(.*?)-->")?;
    let text = re_comments
        .captures_iter(&body)
        .nth(1)
        .map(|caps| caps[1].to_string())
        .unwrap_or_default();

    let mut cnt: HashMap<char, usize> = HashMap::new();
    for c in text.chars() {
        *cnt.entry(c).or_default() += 1;
    }

    for c in text.chars() {
        if cnt.get(&c) == Some(&1) {
            print!("{}", c);
        }
    }

    Ok(())
}
