use custom_error::custom_error;
use std::{io, io::prelude::*};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_URL: &str = "http://localhost:45484";

const HELP: &str = "\
Tiny HTTP client for the Black (blackd) Python code formatter

USAGE:
    blackd-client [OPTIONS]

OPTIONS:
    -h, --help              Print help information
        --url <URL>         URL of blackd server [default: http://localhost:45484]
        --line-length <LEN> Custom max-line-length
    -V, --version           Print version information
";

#[derive(Debug)]
struct AppArgs {
    url: String,
    line_length: Option<i32>,
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    if pargs.contains(["-V", "--version"]) {
        println!("blackd-client v{}", VERSION);
        std::process::exit(0);
    }

    let args = AppArgs {
        url: pargs
            .opt_value_from_str("--url")?
            .unwrap_or_else(|| DEFAULT_URL.to_string()),
        line_length: pargs.opt_value_from_str("--line-length")?,
    };

    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Error: unrecognized arguments: {:?}", remaining);
        std::process::exit(1);
    }

    Ok(args)
}

custom_error! {BlackdError
    Minreq{source: minreq::Error} = "{source}",
    Syntax{details: String} = "Syntax Error: {details}",
    Formatting{details: String} = "Formatting Error: {details}",
    Unknown{status_code: i32, body: String} = "Unknown Error {status_code}: {body}",
}

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    let stdin = read_stdin();
    let result = format(&args, &stdin.unwrap_or_default());
    match result {
        Ok(v) => write_stdout(v.as_bytes()).unwrap(),
        Err(e) => {
            eprint!("Error formatting with blackd-client: {}", e);
            std::process::exit(1);
        }
    }
}

fn write_stdout(buf: &[u8]) -> io::Result<()> {
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());
    writer.write_all(buf)?;
    Ok(())
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

fn format(args: &AppArgs, stdin: &str) -> Result<String, BlackdError> {
    let mut req = minreq::post(&args.url)
        .with_header("X-Fast-Or-Safe", "fast")
        .with_header("Content-Type", "text/plain; charset=utf-8")
        .with_body(stdin);

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

        let args = AppArgs {
            url: server.url(""),
            line_length: None,
        };
        let result = format(&args, "print('Hello World!')");

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

        let args = AppArgs {
            url: server.url(""),
            line_length: Some(120),
        };
        let result = format(&args, "print('Hello World!')");

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

        let args = AppArgs {
            url: server.url(""),
            line_length: None,
        };
        let result = format(&args, body);

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

        let args = AppArgs {
            url: server.url(""),
            line_length: None,
        };
        let result = format(&args, "print('bad syntax'))");

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

        let args = AppArgs {
            url: server.url(""),
            line_length: None,
        };
        let result = format(&args, "print(('bad syntax')");

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

        let args = AppArgs {
            url: server.url(""),
            line_length: None,
        };
        let result = format(&args, "");

        mock.assert();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Unknown Error 418: message"
        );
    }
}
