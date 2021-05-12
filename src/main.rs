use std::{io, io::prelude::*};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let stdin = read_stdin();
    let result = format("http://localhost:45484", stdin.unwrap());
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

fn format(url: &str, stdin: String) -> Result<String> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::MockServer;

    #[test]
    fn format_success_test() {
        let server = MockServer::start();
        let body = "print(\"Hello World!\")";
        let mock = server.mock(|when, then| {
            when.method("POST")
                .path("/")
                .header("X-Fast-Or-Safe", "fast");
            then.status(200).body(body);
        });

        let result = format(server.url("").as_str(), "print('Hello World!')".to_string());

        mock.assert();
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), body);
    }
}
