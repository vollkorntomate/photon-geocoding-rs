use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct PhotonError {
    pub message: String,
}

impl PhotonError {
    pub fn new(message: &str) -> Self {
        PhotonError {
            message: message.to_string(),
        }
    }
}

impl Display for PhotonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl Error for PhotonError {}
