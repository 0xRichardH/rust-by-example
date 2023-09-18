use std::{error::Error, fmt::Display, str::FromStr};

#[derive(Debug)]
pub enum EndpointError {
    InvalidFormat,
}
impl Error for EndpointError {}

impl Display for EndpointError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EndpointError::InvalidFormat => write!(f, "Invalid format"),
        }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Endpoint {
    ip: String,
    port: String,
}

impl FromStr for Endpoint {
    type Err = EndpointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(':').collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(EndpointError::InvalidFormat);
        }

        let (ip, port) = (parts[0].to_string(), parts[1].to_string());
        let endpoint = Endpoint { ip, port };
        Ok(endpoint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), EndpointError> {
        let endpoint: Endpoint = "127.0.0.1:8080".parse()?;
        let expected = Endpoint {
            ip: "127.0.0.1".to_string(),
            port: "8080".to_string(),
        };

        assert_eq!(endpoint, expected);

        Ok(())
    }

    #[test]
    fn it_returns_error_when_input_is_invalid() {
        let endpoint: Result<Endpoint, EndpointError> = "https://127.0.0.1:8080".parse();
        assert!(endpoint.is_err());
        assert_eq!(format!("{}", endpoint.unwrap_err()), "Invalid format");
    }
}
