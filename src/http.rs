//! HTTP transport for remote and containerized language SDKs.

mod content_type;
mod controllers;
mod json_response;
mod openapi;
mod routes;
mod schemas;
mod serve;
mod spec;
mod swagger;

pub use serve::serve;
pub use spec::document as openapi_document;

#[cfg(test)]
mod tests;
