use regex::Regex;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let base = "http://www.pythonchallenge.com/pc/def/linkedlist.php?nothing=";
    let mut nothing = 12345;
    let re = Regex::new(r"next nothing is (\d+)")?;

    loop {
        let url = format!("{base}{nothing}");
        let body = reqwest::blocking::get(&url)?.text()?;
        println!("{} -> {}", nothing, body.trim());

        if body.contains("Divide by two") {
            nothing /= 2;
            continue;
        }

        if let Some(caps) = re.captures(&body) {
            let next_nothing = &caps[1];
            nothing = next_nothing.parse::<i32>()?;
        } else {
            break;
        }
    }

    Ok(())
}
