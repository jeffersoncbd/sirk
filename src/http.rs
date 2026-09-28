//! HTTP transport for remote and containerized language SDKs.

mod handle;
mod serve;
mod types;

pub use serve::serve;

#[cfg(test)]
mod tests;
