//! Line-delimited JSON-RPC transport used by language SDKs.

mod execute;
mod handle;
mod run_agent;
mod serve;
mod types;

pub use serve::serve;

#[cfg(test)]
mod tests;
