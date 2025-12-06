use image::GenericImageView;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.pythonchallenge.com/pc/def/oxygen.png";
    let bytes = reqwest::blocking::get(url)?.bytes()?;
    let img = image::load_from_memory(&bytes)?;

    let (w, h) = img.dimensions();
    let y = h / 2;
    let rgba = img.to_rgba8();

    let mut val = Vec::new();
    for x in (0..w).step_by(7) {
        let p = rgba.get_pixel(x, y);
        let (r, g, b) = (p[0], p[1], p[2]);
        if r == g && g == b {
            val.push(r);
        }
    }

    let text: String = val.iter().map(|&v| v as char).collect();
    println!("{text}");

    let code: [u8; 9] = [105, 110, 116, 101, 103, 114, 105, 116, 121];
    println!("{}", code.iter().map(|&x| x as char).collect::<String>());

    Ok(())
}
