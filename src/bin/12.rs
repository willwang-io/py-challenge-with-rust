use std::error::Error;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let bytes = client
        .get("http://www.pythonchallenge.com/pc/return/evil2.gfx")
        .basic_auth("huge", Some("file"))
        .send()?
        .bytes()?;

    for i in 0..5 {
        let mut file = File::create(format!("{i}.jpg"))?;
        for j in (i..bytes.len()).step_by(5) {
            file.write_all(&[bytes[j]])?;
        }
    }

    Ok(())
}
