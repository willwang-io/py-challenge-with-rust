use regex::Regex;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.pythonchallenge.com/pc/def/equality.html";
    let body = reqwest::blocking::get(url)?.text()?;

    let re_comments = Regex::new(r"(?s)<!--(.*?)-->")?;
    let text = re_comments
        .captures_iter(&body)
        .nth(0)
        .map(|caps| caps[1].to_string())
        .unwrap_or_default();

    let re_pattern = Regex::new(r"[^A-Z][A-Z]{3}([a-z])[A-Z]{3}[^A-Z]")?;

    for caps in re_pattern.captures_iter(&text) {
        print!("{}", &caps[1]);
    }

    Ok(())
}
