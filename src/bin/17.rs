use bzip2::read::BzDecoder;
use regex::Regex;
use reqwest::{blocking::Client, header};
use std::error::Error;
use std::io::{Cursor, Read};
use xmlrpc::{Request, Value};

fn main() -> Result<(), Box<dyn Error>> {
    // For Chrome, open Inspect -> Application -> Storage -> Cookies.
    let base = "http://www.pythonchallenge.com/pc/def/linkedlist.php?busynothing=";
    let client = Client::new();
    let mut busynothing = String::from("12345");

    let re_next = Regex::new(r"next busynothing is (\d+)")?;
    let re_cookie_info = Regex::new(r"(?:^|;\s*)info=([^;]*)")?;

    let mut info_buffer = String::new();

    loop {
        let url = format!("{base}{busynothing}");
        let resp = client.get(&url).send()?;

        for header_val in resp.headers().get_all(header::SET_COOKIE).iter() {
            let s = header_val.to_str()?;
            if let Some(caps) = re_cookie_info.captures(s) {
                if let Some(m) = caps.get(1) {
                    info_buffer.push_str(m.as_str());
                }
            }
        }

        let body = resp.text()?;
        if let Some(caps) = re_next.captures(&body) {
            busynothing = caps[1].to_string();
        } else {
            break;
        }
    }

    let normalized = info_buffer.replace("+", "%20");

    let mut decoder = BzDecoder::new(Cursor::new(urlencoding::decode_binary(
        &normalized.as_bytes(),
    )));
    let mut out = String::new();
    decoder.read_to_string(&mut out)?;
    println!("{out}");

    let url = "http://www.pythonchallenge.com/pc/phonebook.php";

    let request = Request::new("phone").arg("Leopold");
    let response = request.call_url(url)?;

    match response {
        Value::String(s) => println!("{}", s),
        other => println!("{:?}", other),
    }

    let cookies = "info=the flowers are on their way";
    let resp = client
        .get("http://www.pythonchallenge.com/pc/stuff/violin.php")
        .header(header::COOKIE, cookies)
        .send()?;

    let body = resp.text()?;
    println!("{body}");

    Ok(())
}
