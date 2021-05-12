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

    match resp.status_code {
        204 => return Ok(stdin), // input is already well-formatted
        200 => return Ok(resp.as_str()?.to_string()), // input was reformatted by Black
        _ => return Err("".into()),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::MockServer;

    #[test]
    fn test_format_success() {
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
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), body);
    }

    #[test]
    fn test_format_unchanged() {
        let server = MockServer::start();
        let body = "print(\"Already formatted\")";
        let mock = server.mock(|when, then| {
            when.method("POST")
                .path("/")
                .header("X-Fast-Or-Safe", "fast");
            then.status(204);
        });

        let result = format(server.url("").as_str(), body.to_string());

        mock.assert();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), body);
    }

    #[test]
    fn test_format_error() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method("POST");
            then.status(418);
        });

        let result = format(server.url("").as_str(), String::new());

        mock.assert();
        assert!(result.is_err());
    }
}
