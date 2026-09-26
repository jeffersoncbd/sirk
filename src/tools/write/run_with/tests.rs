use super::*;
use crate::tools::write::write;
use std::time::{SystemTime, UNIX_EPOCH};

fn temporary_project() -> std::path::PathBuf {
    let directory = std::env::temp_dir().join(format!(
        "write-tool-test-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&directory).unwrap();
    directory
}

#[test]
fn skip_preserves_existing_files_and_creates_missing_files() {
    let project = temporary_project();
    write_with_options(&project, "nested/file", "original\n", false, true).unwrap();
    write_with_options(&project, "nested/file", "replacement", false, true).unwrap();
    assert_eq!(
        fs::read_to_string(project.join("nested/file")).unwrap(),
        "original\n"
    );
    assert!(write_with_options(&project, "nested/file", "replacement", true, true).is_err());
    assert!(write_with_options(&project, "nested", "content", false, true).is_err());
    assert!(write_with_options(&project, "../outside", "content", false, true).is_err());
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(project.join("nested/file"), project.join("link")).unwrap();
        assert!(write_with_options(&project, "link", "content", false, true).is_err());
    }
    fs::remove_dir_all(project).unwrap();
}

#[test]
fn creates_new_files_and_refuses_existing_ones_without_force() {
    let project = temporary_project();
    write(&project, "nested/deeper/note.txt", "first\n", false).unwrap();
    assert_eq!(
        fs::read_to_string(project.join("nested/deeper/note.txt")).unwrap(),
        "first\n"
    );
    let error = write(&project, "nested/deeper/note.txt", "second", false).unwrap_err();
    assert!(error.contains("force: true"));
    assert_eq!(
        fs::read_to_string(project.join("nested/deeper/note.txt")).unwrap(),
        "first\n"
    );
    fs::remove_dir_all(project).unwrap();
}

#[test]
fn force_replaces_existing_file_contents() {
    let project = temporary_project();
    fs::create_dir_all(project.join("nested")).unwrap();
    fs::write(project.join("nested/note.txt"), "old content").unwrap();
    write(&project, "nested/note.txt", "new content", true).unwrap();
    assert_eq!(
        fs::read_to_string(project.join("nested/note.txt")).unwrap(),
        "new content"
    );
    fs::remove_dir_all(project).unwrap();
}

#[test]
fn rejects_paths_outside_the_project_and_non_files() {
    let project = temporary_project();
    assert!(write(&project, "../outside.txt", "content", false).is_err());
    fs::create_dir(project.join("nested")).unwrap();
    assert!(write(&project, "nested", "content", true).is_err());
    write(&project, "missing/note.txt", "content", false).unwrap();
    assert_eq!(
        fs::read_to_string(project.join("missing/note.txt")).unwrap(),
        "content"
    );
    fs::remove_dir_all(project).unwrap();
}
