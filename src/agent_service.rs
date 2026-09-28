//! Transport-independent agent execution used by protocol servers.

mod execute;
mod run;

pub(crate) use run::run;
