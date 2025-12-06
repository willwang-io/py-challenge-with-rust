use bzip2::read::BzDecoder;
use std::error::Error;
use std::io::{Cursor, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let un_bz: &[u8] = b"BZh91AY&SYA\xaf\x82\r\x00\x00\x01\x01\x80\x02\xc0\x02\x00 \x00!\x9ah3M\x07<]\xc9\x14\xe1BA\x06\xbe\x084";
    let pw_bz: &[u8] =
        b"BZh91AY&SY\x94$|\x0e\x00\x00\x00\x81\x00\x03$ \x00!\x9ah3M\x13<]\xc9\x14\xe1BBP\x91\xf08";

    let mut un_decoder = BzDecoder::new(Cursor::new(un_bz));
    let mut un = String::new();
    un_decoder.read_to_string(&mut un)?;

    let mut pw_decoder = BzDecoder::new(Cursor::new(pw_bz));
    let mut pw = String::new();
    pw_decoder.read_to_string(&mut pw)?;

    println!("{un}");
    println!("{pw}");

    Ok(())
}
