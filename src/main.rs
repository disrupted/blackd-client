use std::{
    io,
    io::{prelude::*, StdinLock},
};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let stdin = read_stdin();
    let _result = format(stdin.unwrap());
    Ok(())
}

// fn read_stdin() -> io::Result<io::StdinLock<'static>> {
//     let stdin = Box::leak(Box::new(io::stdin()));
//     // for line in stdin.lock().lines() {
//     //     println!("{}", line?);
//     // }
//     Ok(stdin.lock())
// }

fn read_stdin() -> Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    {
        let mut stdin_lock = stdin.lock(); // We get `StdinLock` here.
        stdin_lock.read_to_string(&mut buffer)?;
    } // `StdinLock` is dropped here.
    Ok(buffer)
}

fn format(stdin: String) -> Result<String> {
    // println!("{}", stdin);
    let client = reqwest::blocking::Client::new();
    let mut resp = client
        .post("http://localhost:45484")
        .body("print('bli bla blub')")
        .send()?;

    println!("res = {:?}", resp);
    if resp.status() == 204 {
        return Ok(stdin);
    }

    let mut body = String::new();
    resp.read_to_string(&mut body)?;
    println!("{}", body);

    Ok(body)
}
