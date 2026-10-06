use super::{History, flow_id::flow_id};
use std::{fs, path::Path};

pub fn new_flow(directory: &Path) -> Result<String, String> {
    let directory = directory
        .canonicalize()
        .map_err(|error| error.to_string())?;
    let history = directory.join("history");
    fs::create_dir_all(&history).map_err(|error| error.to_string())?;
    loop {
        let flow_id = flow_id()?;
        let flow_directory = history.join(&flow_id);
        match fs::create_dir(&flow_directory) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error.to_string()),
        }
        let path = History::flow_path(&directory, &flow_id)?;
        let usage_path = History::usage_path(&directory, &flow_id)?;
        let conversations = flow_directory.join("conversations");
        let flow_file = fs::File::create(path).map_err(|error| error.to_string())?;
        let usage_file = fs::File::create(usage_path).map_err(|error| error.to_string())?;
        fs::create_dir(&conversations).map_err(|error| error.to_string())?;
        flow_file.sync_all().map_err(|error| error.to_string())?;
        usage_file.sync_all().map_err(|error| error.to_string())?;
        fs::File::open(&conversations)
            .and_then(|file| file.sync_all())
            .map_err(|error| error.to_string())?;
        fs::File::open(&flow_directory)
            .and_then(|file| file.sync_all())
            .map_err(|error| error.to_string())?;
        fs::File::open(&history)
            .and_then(|file| file.sync_all())
            .map_err(|error| error.to_string())?;
        return Ok(flow_id);
    }
}
