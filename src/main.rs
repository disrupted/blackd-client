use std::{io, io::prelude::*};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let stdin = read_stdin();
    let result = format("http://localhost:45484".to_string(), stdin.unwrap());
    print!("{}", result.unwrap());
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

fn format(url: String, stdin: String) -> Result<String> {
    let resp = minreq::post(url)
        .with_header("X-Fast-Or-Safe", "fast")
        .with_body(stdin.as_str())
        .send()?;

    let result = match resp.status_code {
        204 => stdin,                      // input is already well-formatted
        200 => resp.as_str()?.to_string(), // input was reformatted by Black
        _ => "".to_string(),
    };

    Ok(result)
}
