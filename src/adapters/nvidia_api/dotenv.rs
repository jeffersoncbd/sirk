use std::{io, path::Path};

use super::{NvidiaApiAdapter, nonempty::nonempty};
use crate::harness::{HarnessAdapter, HarnessError};

impl NvidiaApiAdapter {
    pub(super) fn dotenv_value(
        &self,
        directory: &Path,
        key: &str,
    ) -> Result<Option<String>, HarnessError> {
        let dotenv = directory.join(".env");
        let variables = match dotenvy::from_path_iter(&dotenv) {
            Ok(variables) => variables,
            Err(dotenvy::Error::Io(error)) if error.kind() == io::ErrorKind::NotFound => {
                return Ok(None);
            }
            Err(error) => {
                return Err(HarnessError::InvalidConfiguration {
                    adapter: self.id(),
                    message: format!("cannot read `{}`: {error}", dotenv.display()),
                });
            }
        };
        for variable in variables {
            let (name, value) = variable.map_err(|error| HarnessError::InvalidConfiguration {
                adapter: self.id(),
                message: format!("cannot parse `{}`: {error}", dotenv.display()),
            })?;
            if name == key {
                return Ok(nonempty(value));
            }
        }
        Ok(None)
    }
}
