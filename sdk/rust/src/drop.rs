use crate::Sirk;

impl Drop for Sirk {
    fn drop(&mut self) {
        self.input.take();
        let _ = self.child.wait();
    }
}
