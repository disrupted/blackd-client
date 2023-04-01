use custom_error::custom_error;

custom_error! {pub BlackdError
    Minreq{source: minreq::Error} = "{source}",
    Syntax{details: String} = "Syntax Error: {details}",
    Formatting{details: String} = "Formatting Error: {details}",
    Unknown{status_code: i32, body: String} = "Unknown Error {status_code}: {body}",
}
