use std::path::PathBuf;

#[cfg(unix)]
pub(super) fn path_from_bytes(bytes: &[u8]) -> Result<PathBuf, String> {
    use std::os::unix::ffi::OsStrExt;
    Ok(PathBuf::from(std::ffi::OsStr::from_bytes(bytes)))
}

#[cfg(not(unix))]
pub(super) fn path_from_bytes(bytes: &[u8]) -> Result<PathBuf, String> {
    std::str::from_utf8(bytes)
        .map(PathBuf::from)
        .map_err(|e| format!("TREE received a path unsupported on this platform: {e}"))
}
