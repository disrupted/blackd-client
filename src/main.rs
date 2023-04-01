mod args;
mod config;
mod error;
mod io_utils;

use args::AppArgs;
use config::Config;
use error::BlackdError;
use io_utils::{read_stdin, write_stdout};
use minreq;

fn main() {
    let config = Config::load(None);
    let args = AppArgs::parse();

    let stdin = read_stdin();
    let result = format(&config, &args, &stdin.unwrap_or_default());
    match result {
        Ok(v) => write_stdout(v.as_bytes()).unwrap(),
        Err(e) => {
            eprint!("Error formatting with blackd-client: {}", e);
            std::process::exit(1);
        }
    }
}

fn format(config: &Config, args: &AppArgs, stdin: &str) -> Result<String, BlackdError> {
    let mut req = minreq::post(&args.url)
        .with_header("X-Fast-Or-Safe", "fast")
        .with_header("Content-Type", "text/plain; charset=utf-8")
        .with_body(stdin);

    if let Some(tool) = &config.tool {
        if let Some(black) = &tool.black {
            if let Some(target_version) = &black.target_version {
                req = req.with_header("X-Target-Version", target_version.join(","));
            }

            if let Some(line_length) = &black.line_length {
                req = req.with_header("X-Line-Length", line_length.to_string());
            }
        }
    }

    // CLI args override config
    if let Some(line_length) = &args.line_length {
        req = req.with_header("X-Line-Length", line_length.to_string());
    }

    let resp = req.send()?;

    let body = resp.as_str()?.to_string();
    match resp.status_code {
        200 => Ok(body),              // input was reformatted by Black
        204 => Ok(stdin.to_string()), // input is already well-formatted
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

        let config = Config { tool: None };
        let args = AppArgs {
            url: server.url(""),
            line_length: None,
        };
        let result = format(&config, &args, "print('Hello World!')");

        mock.assert();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), body);
    }

    #[test]
    fn test_format_line_length() {
        let server = MockServer::start();
        let body = "print(\"Hello World!\")";
        let mock = server.mock(|when, then| {
            when.method("POST")
                .path("/")
                .header("X-Fast-Or-Safe", "fast")
                .header("X-Line-Length", "120");
            then.status(200).body(body);
        });

        let config = Config { tool: None };
        let args = AppArgs {
            url: server.url(""),
            line_length: Some(120),
        };
        let result = format(&config, &args, "print('Hello World!')");

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

        let config = Config { tool: None };
        let args = AppArgs {
            url: server.url(""),
            line_length: None,
        };
        let result = format(&config, &args, body);

        mock.assert();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), body);
    }

    #[test]
    fn test_syntax_error() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method("POST");
            then.status(400)
                .body("Cannot parse: 1:6: print('bad syntax'))");
        });

        let config = Config { tool: None };
        let args = AppArgs {
            url: server.url(""),
            line_length: None,
        };
        let result = format(&config, &args, "print('bad syntax'))");

        mock.assert();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Syntax Error: Cannot parse: 1:6: print('bad syntax'))"
        );
    }

    #[test]
    fn test_formatting_error() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method("POST");
            then.status(500)
                .body("('EOF in multi-line statement', (2, 0))");
        });

        let config = Config { tool: None };
        let args = AppArgs {
            url: server.url(""),
            line_length: None,
        };
        let result = format(&config, &args, "print(('bad syntax')");

        mock.assert();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Formatting Error: ('EOF in multi-line statement', (2, 0))"
        );
    }

    #[test]
    fn test_unknown_error() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method("POST");
            then.status(418).body("message");
        });

        let config = Config { tool: None };
        let args = AppArgs {
            url: server.url(""),
            line_length: None,
        };
        let result = format(&config, &args, "");

        mock.assert();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Unknown Error 418: message"
        );
    }
}
