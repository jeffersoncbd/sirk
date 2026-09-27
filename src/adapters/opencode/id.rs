// Keep the trait method in its own file while sharing one trait implementation.
macro_rules! adapter_id {
    () => {
        fn id(&self) -> &'static str {
            "opencode"
        }
    };
}

pub(super) use adapter_id;
