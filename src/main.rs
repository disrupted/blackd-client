use std::{io, io::prelude::*};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let _result = read_stdin();
    let _result = format();
    Ok(())
}

fn read_stdin() -> Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    // call blackd

    // TODO
    if true == true {
        for line in lines {
            println!("{}", line?);
        }
    }
    Ok(())
}

fn format() -> Result<()> {
    let client = reqwest::blocking::Client::new();
    let mut resp = client
        .post("http://localhost:45484")
        .body("print('bli bla blub')")
        .send()?;

    // println!("res = {:?}", resp);

    let mut body = String::new();
    resp.read_to_string(&mut body)?;
    println!("{}", body);

    Ok(())
}
