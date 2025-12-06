use regex::Regex;
use std::error::Error;
use std::io::{Cursor, Read};
use zip::ZipArchive;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.pythonchallenge.com/pc/def/channel.zip";
    let bytes = reqwest::blocking::get(url)?.bytes()?;
    let cursor = Cursor::new(bytes);
    let mut archive = ZipArchive::new(cursor)?;

    let mut nothing = String::from("90052"); // Get it from the readme.txt inside the zip
    let re_next = Regex::new(r"Next nothing is (\d+)")?;

    loop {
        let filename = format!("{nothing}.txt");
        let mut file = archive.by_name(&filename)?;
        print!("{}", file.comment());

        let mut content = String::new();
        file.read_to_string(&mut content)?;

        if let Some(caps) = re_next.captures(&content) {
            nothing = caps[1].to_string();
        } else {
            break;
        }
    }

    Ok(())
}
