use super::History;

impl Drop for History {
    fn drop(&mut self) {
        // Explicitly release even if a concurrently spawned child briefly inherited the fd.
        let _ = self._lock.unlock();
    }
}
