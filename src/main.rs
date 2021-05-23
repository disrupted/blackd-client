use std::{io, io::prelude::*};

const BLACKD_DEFAULT_URL: &str = "http://localhost:45484";

fn main() {
    let stdin = read_stdin();
    let result = format(BLACKD_DEFAULT_URL, stdin.unwrap());
    match result {
        Ok(v) => print!("{}", v),
        Err(e) => print!("Error formatting with blackd-client: {}", e),
    }
}

fn read_stdin() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer)?;
    }
    Ok(buffer)
}

fn format(url: &str, stdin: String) -> Result<String, minreq::Error> {
    let resp = minreq::post(url)
        .with_header("X-Fast-Or-Safe", "fast")
        .with_header("Content-Type", "text/plain; charset=utf-8")
        .with_body(stdin.as_str())
        .send()?;

    match resp.status_code {
        204 => Ok(stdin),                      // input is already well-formatted
        200 => Ok(resp.as_str()?.to_string()), // input was reformatted by Black
        _ => Err(minreq::Error::Other("Error")),
    }
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
