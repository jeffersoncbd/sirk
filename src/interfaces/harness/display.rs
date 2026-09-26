use super::HarnessError;
use std::fmt;

impl fmt::Display for HarnessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedOption { adapter, option } => {
                write!(formatter, "adapter `{adapter}` does not support `{option}`")
            }
            Self::MissingModel { adapter } => {
                write!(formatter, "adapter `{adapter}` requires a model")
            }
            Self::InvalidResponse { adapter, message } => write!(
                formatter,
                "adapter `{adapter}` returned an invalid response: {message}"
            ),
            Self::InvalidConfiguration { adapter, message } => write!(
                formatter,
                "adapter `{adapter}` has invalid configuration: {message}"
            ),
        }
    }
}
