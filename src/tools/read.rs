//! READ: return the UTF-8 contents of a file inside the execution directory.
mod component;
mod component_match;
mod components;
mod enumerate;
mod enumerated_content;
mod ignore;
mod ignored;
mod run;

pub use enumerate::enumerate;
pub use enumerated_content::enumerated_content;
pub use run::read;
