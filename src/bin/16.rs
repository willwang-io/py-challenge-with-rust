use image::{ImageBuffer, Rgba};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let bytes = client
        .get("http://www.pythonchallenge.com/pc/return/mozart.gif")
        .basic_auth("huge", Some("file"))
        .send()?
        .bytes()?;

    let img = image::load_from_memory(&bytes)?.to_rgba8();
    let (w, h) = img.dimensions();

    let y_ref = 0;
    let mut bar_color = img.get_pixel(0, y_ref);
    let mut max_run = 0;
    let mut run_color = img.get_pixel(0, y_ref);
    let mut run_len = 1;

    for x in 1..w {
        let p = img.get_pixel(x, y_ref);
        if p == run_color {
            run_len += 1;
            if run_len > max_run {
                max_run = run_len;
                bar_color = run_color;
            }
        } else {
            run_color = p;
            run_len = 1;
        }
    }

    let mut out = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(w, h);
    for y in 0..h {
        let mut row: Vec<Rgba<u8>> = (0..w).map(|x| *img.get_pixel(x, y)).collect();
        if let Some(idx) = row.iter().position(|c| c == bar_color) {
            row.rotate_left(idx);
        }
        for x in 0..w {
            out.put_pixel(x, y, row[x as usize]);
        }
    }

    out.save("16.gif")?;

    Ok(())
}
