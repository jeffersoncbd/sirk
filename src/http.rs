//! HTTP transport for remote and containerized language SDKs.

mod handle;
mod openapi;
mod serve;
mod swagger;
mod types;

pub use serve::serve;

#[cfg(test)]
mod tests;
