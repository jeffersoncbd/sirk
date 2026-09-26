use super::{History, SEPARATOR, TITLE, reserved::reserved};
use std::{
    fs::{self, File},
    io::Write,
};

impl History {
    pub fn save(&self) -> Result<(), String> {
        let metadata = serde_yaml::to_string(&self.snapshot).map_err(|e| e.to_string())?;
        let mut source = format!("{TITLE}{metadata}---\n");
        for (index, blocks) in self.steps.iter().enumerate() {
            let label = self.labels.get(index).cloned().unwrap_or_else(|| {
                format!(
                    "Step {} — {}",
                    index + 1,
                    self.snapshot.workflow.steps[index].name()
                )
            });
            source.push_str(&format!("{SEPARATOR}\n{label}\n\n"));
            for block in blocks {
                source.push_str(block.marker());
                source.push('\n');
                for line in block.text().split('\n') {
                    if reserved(line) {
                        source.push('\\');
                    }
                    source.push_str(line);
                    source.push('\n');
                }
                source.push('\n');
            }
        }
        let temporary = self.path.with_extension("log.tmp");
        let mut file = File::create(&temporary).map_err(|e| e.to_string())?;
        file.write_all(source.as_bytes())
            .and_then(|_| file.sync_all())
            .map_err(|e| e.to_string())?;
        fs::rename(&temporary, &self.path).map_err(|e| e.to_string())?;
        File::open(self.path.parent().unwrap())
            .and_then(|f| f.sync_all())
            .map_err(|e| e.to_string())
    }
}
