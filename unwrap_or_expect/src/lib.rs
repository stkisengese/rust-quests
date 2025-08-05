pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match server {
        Ok(url) => {
            match security_level {
                Security::UnexpectedUrl => panic!("{}", url),
                _ => url.to_string(),
            }
        },
        Err(msg) => {
            match security_level {
                Security::Unknown => panic!("called `Result::unwrap()` on an `Err` value: \"{}\"", msg),
                Security::Message => panic!("ERROR: program stops"),
                Security::Warning => "WARNING: check the server".to_string(),
                Security::NotFound => format!("Not found: {}", msg),
                Security::UnexpectedUrl => msg.to_string(),
            }
        }
    }
}