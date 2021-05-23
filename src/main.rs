use clap::{crate_version, AppSettings, Clap};
use custom_error::custom_error;
use std::{io, io::prelude::*};

/// Tiny HTTP client for the Black (blackd) Python code formatter
#[derive(Clap)]
#[clap(version = crate_version!())]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(long, default_value = "http://localhost:45484")]
    url: String,
}

custom_error! {BlackdError
    Minreq{source: minreq::Error} = "{source}",
    Syntax{details: String} = "Syntax Error: {details}",
    Formatting{details: String} = "Formatting Error: {details}",
    Unknown{status_code: i32, body: String} = "Unknown Error: {status_code}",
}

fn main() {
    let opts: Opts = Opts::parse();
    let stdin = read_stdin();
    let result = format(opts.url, stdin.unwrap());
    match result {
        Ok(v) => print!("{}", v),
        Err(e) => {
            eprint!("Error formatting with blackd-client: {}", e);
            std::process::exit(1);
        }
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

fn format(url: String, stdin: String) -> Result<String, BlackdError> {
    let resp = minreq::post(url)
        .with_header("X-Fast-Or-Safe", "fast")
        .with_header("Content-Type", "text/plain; charset=utf-8")
        .with_body(stdin.as_str())
        .send()?;

    let body = resp.as_str()?.to_string();
    match resp.status_code {
        200 => Ok(body),  // input was reformatted by Black
        204 => Ok(stdin), // input is already well-formatted
        400 => Err(BlackdError::Syntax { details: body }),
        500 => Err(BlackdError::Formatting { details: body }),
        _ => Err(BlackdError::Unknown {
            status_code: resp.status_code,
            body,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::MockServer;
    use pretty_assertions::assert_eq;

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

        let result = format(server.url(""), "print('Hello World!')".to_string());

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

        let result = format(server.url(""), body.to_string());

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

        let result = format(server.url(""), String::new());

        mock.assert();
        assert!(result.is_err());
    }
}
