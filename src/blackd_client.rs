use crate::app_args::AppArgs;
use crate::config::Config;
use crate::error::BlackdError;
use minreq;

pub struct BlackdClient<'a> {
    url: &'a str,
}

impl<'a> BlackdClient<'a> {
    pub fn new(url: &'a str) -> Self {
        BlackdClient { url }
    }

    pub fn format(
        &self,
        config: &Config,
        args: &AppArgs,
        stdin: &str,
    ) -> Result<String, BlackdError> {
        let mut req = minreq::post(&self.url)
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
