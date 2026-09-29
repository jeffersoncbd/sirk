use super::{History, valid_flow_id};
use std::path::Path;

pub fn validate_flow(directory: &Path, flow_id: &str) -> Result<(), String> {
    if !valid_flow_id(flow_id) {
        return Err("invalid flow ID".to_owned());
    }
    let path = History::flow_path(directory, flow_id)?;
    if path.is_file() {
        Ok(())
    } else {
        Err(format!("unknown flow ID `{flow_id}`"))
    }
}
