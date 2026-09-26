use super::Block;

impl Block {
    pub fn text(&self) -> &str {
        match self {
            Self::Ask(text)
            | Self::Input(text)
            | Self::Output(text)
            | Self::Tree(text)
            | Self::Read(text)
            | Self::Edit(text)
            | Self::Delete(text) => text,
        }
    }
}
