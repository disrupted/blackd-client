use std::{io, io::prelude::*};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let stdin = read_stdin();
    let _result = format(stdin.unwrap());
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

fn format(stdin: String) -> Result<()> {
    let resp = minreq::post("http://localhost:45484")
        .with_header("X-Fast-Or-Safe", "fast")
        .with_body(stdin.as_str())
        .send()?;

    // input is already well-formatted
    if resp.status_code == 204 {
        print!("{}", stdin);
        return Ok(());
    }

    // input was reformatted by Black
    if resp.status_code == 200 {
        print!("{}", resp.as_str()?);
    }

    Ok(())
}
