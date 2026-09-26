use super::Block;

impl Block {
    pub fn marker(&self) -> &str {
        match self {
            Self::Ask(_) => "==> ASK",
            Self::Input(_) => "==> INPUT",
            Self::Output(_) => "<== OUTPUT",
            Self::Tree(_) => "==> TREE",
            Self::Read(_) => "==> READ",
            Self::Edit(_) => "==> EDIT",
            Self::Delete(_) => "==> DELETE",
        }
    }
}
