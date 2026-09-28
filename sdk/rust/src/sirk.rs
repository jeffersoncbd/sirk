use crate::transport::Transport;
use std::path::PathBuf;

pub struct Sirk {
    pub(super) directory: PathBuf,
    pub(super) local_directory: PathBuf,
    pub(super) transport: Box<dyn Transport>,
}
