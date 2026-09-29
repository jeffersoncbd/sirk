//! HTTP transport for remote and containerized language SDKs.

mod controllers;
mod openapi;
mod routes;
mod serve;
mod swagger;
mod types;

pub use serve::serve;

#[cfg(test)]
mod tests;
