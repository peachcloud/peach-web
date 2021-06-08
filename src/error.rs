//!! different types of PeachWebError

use peach_lib::{serde_json, serde_yaml};
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum PeachWebError {
    #[snafu(display("Error loading serde json"))]
    Serde { source: serde_json::error::Error },
    #[snafu(display("Error loading peach-config yaml"))]
    YamlError { source: serde_yaml::Error },
    #[snafu(display("{}", msg))]
    FailedToRegisterDynDomain { msg: String },
}

impl From<serde_json::error::Error> for PeachWebError {
    fn from(err: serde_json::error::Error) -> PeachWebError {
        PeachWebError::Serde { source: err }
    }
}

impl From<serde_yaml::Error> for PeachWebError {
    fn from(err: serde_yaml::Error) -> PeachWebError {
        PeachWebError::YamlError { source: err }
    }
}
