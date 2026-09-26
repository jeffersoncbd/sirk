use crate::{history::History, input::TerminalInput};
use std::{collections::BTreeMap, path::Path};

use super::{execute::execute, execution::continue_with};

pub fn resume(path: &Path) -> Result<BTreeMap<String, String>, String> {
    continue_with(&mut History::open(path)?, execute, &mut TerminalInput)
}
