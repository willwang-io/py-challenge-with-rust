use std::error::Error;
use xmlrpc::{Request, Value};

fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.pythonchallenge.com/pc/phonebook.php";

    let request = Request::new("phone").arg("Bert"); // This clue is from evil4.jpg
    let response = request.call_url(url)?;

    match response {
        Value::String(s) => println!("{}", s),
        other => println!("{:?}", other),
    }

    Ok(())
}
