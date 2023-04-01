use std::io::{self, prelude::*};

pub fn write_stdout(buf: &[u8]) -> io::Result<()> {
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());
    writer.write_all(buf)?;
    Ok(())
}

pub fn read_stdin() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer)?;
    }
    Ok(buffer)
}
