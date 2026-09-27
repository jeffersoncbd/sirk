use std::{
    io::BufReader,
    process::{Child, ChildStdin, ChildStdout},
};

pub struct Sirk {
    pub(super) child: Child,
    pub(super) input: Option<ChildStdin>,
    pub(super) output: BufReader<ChildStdout>,
    pub(super) next_id: u64,
}
