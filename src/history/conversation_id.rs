use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

static COUNTER: AtomicU64 = AtomicU64::new(0);

pub(super) fn conversation_id() -> Result<String, String> {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_nanos();
    let count = COUNTER.fetch_add(1, Ordering::Relaxed);
    Ok(format!(
        "conversation-{time:x}-{:x}-{count:x}",
        std::process::id()
    ))
}
