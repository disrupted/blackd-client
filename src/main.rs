use std::{io, io::prelude::*};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let stdin = read_stdin();
    let _result = format(stdin.unwrap());
    Ok(())
}

fn read_stdin() -> Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer)?;
    }
    Ok(buffer)
}

fn format(stdin: String) -> Result<String> {
    let client = reqwest::blocking::Client::new();
    let reqbody: String = String::from(&stdin);
    let mut resp = client
        .post("http://localhost:45484")
        .header("X-Fast-Or-Safe", "fast")
        .body(reqbody)
        .send()?;

    // input is already well-formatted
    if resp.status() == 204 {
        print!("{}", stdin);
        return Ok(stdin);
    }

    let mut body = String::new();
    // input was reformatted by Black
    if resp.status() == 200 {
        resp.read_to_string(&mut body)?;
        print!("{}", body);
    }

    Ok(body)
}
