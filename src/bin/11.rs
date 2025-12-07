use image::{GenericImageView, Pixel, RgbImage};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.pythonchallenge.com/pc/return/cave.jpg";
    let client = reqwest::blocking::Client::new();
    let bytes = client
        .get(url)
        .basic_auth("huge", Some("file"))
        .send()?
        .bytes()?;

    let img = image::load_from_memory(&bytes)?;
    let (w, h) = img.dimensions();
    let mut even = RgbImage::new(w / 2, h / 2);
    let mut odd = RgbImage::new(w / 2, h / 2);

    for x in 0..w {
        for y in 0..h {
            let p = img.get_pixel(x, y).to_rgb();
            if (x + y) % 2 == 0 {
                even.put_pixel(x / 2, y / 2, p);
            } else {
                odd.put_pixel(x / 2, y / 2, p);
            }
        }
    }

    even.save("even.png")?;
    odd.save("odd.png")?;

    Ok(())
}
