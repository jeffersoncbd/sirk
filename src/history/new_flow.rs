use super::{History, flow_id::flow_id};
use std::{
    fs::{self, OpenOptions},
    path::Path,
};

pub fn new_flow(directory: &Path) -> Result<String, String> {
    let directory = directory
        .canonicalize()
        .map_err(|error| error.to_string())?;
    let history = directory.join("history");
    fs::create_dir_all(&history).map_err(|error| error.to_string())?;
    loop {
        let flow_id = flow_id()?;
        let path = History::flow_path(&directory, &flow_id)?;
        match OpenOptions::new().write(true).create_new(true).open(path) {
            Ok(file) => {
                file.sync_all().map_err(|error| error.to_string())?;
                std::fs::File::open(&history)
                    .and_then(|file| file.sync_all())
                    .map_err(|error| error.to_string())?;
                return Ok(flow_id);
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error.to_string()),
        }
    }
}
