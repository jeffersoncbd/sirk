use std::{collections::BTreeSet, path::Path};

use super::git::git;

pub(super) fn tree_ignored(root: &Path) -> Result<BTreeSet<Vec<u8>>, String> {
    let result = git(
        root,
        &[
            "ls-files",
            "--cached",
            "--others",
            "--ignored",
            "--exclude-per-directory=.treeignore",
            "-z",
            "--",
            ".",
        ],
    )?;
    if !result.status.success() {
        return Err(format!(
            "GIT-STATUS-TREE .treeignore evaluation failed: {}",
            result.status
        ));
    }
    Ok(result
        .stdout
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .map(<[u8]>::to_vec)
        .collect())
}
