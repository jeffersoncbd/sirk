//! READ: return the UTF-8 contents of a file inside the execution directory.
mod allowed;
mod component;
mod component_match;
mod components;
mod enumerate;
mod enumerate_from;
mod enumerated_content;
mod ignore;
mod ignored;
mod page;
mod request;
mod run;

pub use allowed::allowed;
pub use enumerate::enumerate;
pub use enumerate_from::enumerate_from;
pub use enumerated_content::enumerated_content;
pub use page::page;
pub use run::read;

#[cfg(test)]
#[path = "read/page_tests.rs"]
mod page_tests;
