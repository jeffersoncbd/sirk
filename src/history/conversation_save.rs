use super::Conversation;
use std::{
    fs::{self, OpenOptions},
    io::Write,
};

impl Conversation {
    pub(crate) fn save(&self) -> Result<(), String> {
        let mut content =
            serde_json::to_vec_pretty(&self.data).map_err(|error| error.to_string())?;
        content.push(b'\n');
        let temporary = self.path.with_extension("json.tmp");
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&temporary)
            .map_err(|error| error.to_string())?;
        file.write_all(&content)
            .and_then(|_| file.sync_all())
            .map_err(|error| error.to_string())?;
        fs::rename(&temporary, &self.path).map_err(|error| error.to_string())?;
        fs::File::open(
            self.path
                .parent()
                .ok_or("missing conversations directory")?,
        )
        .and_then(|file| file.sync_all())
        .map_err(|error| error.to_string())
    }
}
