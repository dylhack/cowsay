#![allow(dead_code)]

pub type Result<T> = std::result::Result<T, String>;
pub type CommandResult = Result<Response>;

pub enum Response {
    // Don't respond (the command will do it)
    Ignore,
    // Respond without a message
    Success,
    // Respond with a message (bool: ephemeral)
    Ok(String, bool),
    // User error (tell the user something went wrong that they can fix)
    Warning(String),
}

impl Response {
    pub fn warning(msg: &str) -> Result<Self> {
        Ok(Response::Warning(msg.to_string()))
    }

    pub fn ok(msg: String, ephemeral: bool) -> Result<Self> {
        Ok(Response::Ok(msg, ephemeral))
    }

    pub fn success() -> Result<Self> {
        Ok(Response::Success)
    }

    pub fn ignore() -> Result<Self> {
        Ok(Response::Ignore)
    }

    pub fn err<T: ToString>(msg: T) -> Result<Self> {
        Err(msg.to_string())
    }
}
