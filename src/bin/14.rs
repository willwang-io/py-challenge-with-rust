use image::{GenericImageView, RgbaImage};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let bytes = client
        .get("http://www.pythonchallenge.com/pc/return/wire.png")
        .basic_auth("huge", Some("file"))
        .send()?
        .bytes()?;

    let img = image::load_from_memory(&bytes)?;
    let (w, _) = img.dimensions();
    let n = w.isqrt();
    let mut output = RgbaImage::new(n, n);

    let mut top = 0;
    let mut bottom = n - 1;
    let mut left = 0;
    let mut right = n - 1;
    let mut i = 0;

    while top <= bottom && left <= right && i < w {
        for j in (top..=bottom).rev() {
            output.put_pixel(j, left, img.get_pixel(i, 0));
            i += 1
        }
        left += 1;

        for j in left..=right {
            output.put_pixel(top, j, img.get_pixel(i, 0));
            i += 1;
        }
        top += 1;

        for j in top..=bottom {
            output.put_pixel(j, right, img.get_pixel(i, 0));
            i += 1;
        }
        right -= 1;

        for j in (left..=right).rev() {
            output.put_pixel(bottom, j, img.get_pixel(i, 0));
            i += 1;
        }
        bottom -= 1;
    }

    output.save("14.png")?;

    Ok(())
}
