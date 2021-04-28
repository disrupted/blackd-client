use std::{io, io::prelude::*};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let _stdin = read_stdin();
    let _result = format();
    Ok(())
}

fn read_stdin() -> io::Result<io::StdinLock<'static>> {
    let stdin = Box::leak(Box::new(io::stdin()));
    for line in stdin.lock().lines() {
        println!("{}", line?);
    }
    Ok(stdin.lock())
}

fn format() -> Result<String> {
    let client = reqwest::blocking::Client::new();
    let mut resp = client
        .post("http://localhost:45484")
        .body("print('bli bla blub')")
        .send()?;

    // println!("res = {:?}", resp);

    let mut body = String::new();
    resp.read_to_string(&mut body)?;
    println!("{}", body);

    Ok(body)
}
