use serde_pickle::{DeOptions, from_slice};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.pythonchallenge.com/pc/def/banner.p";
    let bytes = reqwest::blocking::get(url)?.bytes()?;
    let banner: Vec<Vec<(String, usize)>> = from_slice(&bytes, DeOptions::new())?;

    for line in banner {
        for (ch, cnt) in line {
            print!("{}", ch.repeat(cnt));
        }
        println!();
    }

    Ok(())
}
