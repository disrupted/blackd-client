/// A tiny HTTP client for the Black (blackd) Python code formatter.
///
/// # Usage
///
/// ```
/// blackd-client [OPTIONS]
/// ```
///
/// # Options
///
/// * `-h`, `--help`: Print help information
/// * `--url <URL>`: URL of blackd server [default: http://localhost:45484]
/// * `--line-length <LEN>`: Custom max-line-length
/// * `-V`, `--version`: Print version information
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
pub struct AppArgs {
    /// URL of the Blackd server.
    pub url: String,
    /// Custom max-line-length.
    pub line_length: Option<i32>,
}

impl AppArgs {
    /// Parses the command-line arguments and returns an `AppArgs` instance.
    pub fn parse() -> Self {
        let mut pargs = pico_args::Arguments::from_env();

        if pargs.contains(["-h", "--help"]) {
            print!("{}", HELP);
            std::process::exit(0);
        }

        if pargs.contains(["-V", "--version"]) {
            println!("blackd-client v{}", env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        }

        let url = if let Some(url) = pargs.opt_value_from_str("--url").unwrap() {
            url
        } else {
            "http://localhost:45484".to_string()
        };
        let args = AppArgs {
            url: url,
            line_length: pargs.opt_value_from_str("--line-length").unwrap_or(None),
        };

        let remaining = pargs.finish();
        if !remaining.is_empty() {
            eprintln!("Error: unrecognized arguments: {:?}", remaining);
            std::process::exit(1);
        }

        args
    }
}
