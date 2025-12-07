use image::RgbImage;
use regex::Regex;
use std::error::Error;

fn parse_numbers(s: &str) -> Vec<u32> {
    s.split(',')
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.pythonchallenge.com/pc/return/good.html";
    let client = reqwest::blocking::Client::new();
    let body = client
        .get(url)
        .basic_auth("huge", Some("file"))
        .send()?
        .text()?;

    let re_first = Regex::new(r"first:\s*([\d,\s]+)\s*second:")?;
    let re_second = Regex::new(r"second:\s*([\d,\s]+)")?;

    let cap_first = re_first.captures(&body).ok_or("first not found")?;
    let cap_second = re_second.captures(&body).ok_or("second not found")?;

    let nums_first = parse_numbers(&cap_first[1]);
    let nums_second = parse_numbers(&cap_second[1]);

    let mut img = RgbImage::new(500, 500);

    for chunk in nums_first.chunks(2) {
        img.put_pixel(chunk[0], chunk[1], image::Rgb([255, 0, 0]));
    }

    for chunk in nums_second.chunks(2) {
        img.put_pixel(chunk[0], chunk[1], image::Rgb([255, 0, 0]));
    }

    img.save("09.png")?;
    println!("save to 09.png");

    Ok(())
}
