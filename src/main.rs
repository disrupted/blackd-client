use std::{io, io::prelude::*};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn main() -> Result<()> {
    for line in io::stdin().lock().lines() {
        println!("{}", line?);
    }
    Ok(())
}
