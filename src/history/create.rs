use super::{History, Snapshot, valid_flow_id};

impl History {
    pub fn create(snapshot: Snapshot, flow_id: &str) -> Result<Self, String> {
        if !valid_flow_id(flow_id) {
            return Err("invalid flow ID".to_owned());
        }
        let path = Self::flow_path(&snapshot.directory, flow_id)?;
        if !path.is_file() {
            return Err(format!("unknown flow ID `{flow_id}`"));
        }
        let lock = Self::lock(&path)?;
        let history = Self {
            path,
            snapshot,
            blocks: Vec::new(),
            _lock: lock,
        };
        Ok(history)
    }
}
