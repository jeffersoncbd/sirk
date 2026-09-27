mod default;
mod id;
mod invocation;
mod new;

#[cfg(test)]
mod tests;

/// Translates provider-neutral requests into OpenCode CLI invocations.
#[derive(Debug)]
pub struct OpenCodeAdapter {
    executable: String,
}
