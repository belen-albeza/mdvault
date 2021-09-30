use mdvault::{Note, Vault};
use std::path::PathBuf;

fn is_there_note_with_rel_path(notes: &[Note], path_str: &str) -> bool {
    let path = PathBuf::from(path_str);
    notes
        .iter()
        .find(|&note| note.rel_path() == &path)
        .is_some()
}

#[test]
fn vault_scans_file_tree_on_creation() {
    let base_path = PathBuf::from("tests/fixtures/basic");
    let vault = Vault::new(&base_path);

    assert_eq!(vault.base_path, base_path);
    assert_eq!(vault.notes.len(), 2);

    assert!(is_there_note_with_rel_path(&vault.notes, "foo/bar.md"));
    assert!(is_there_note_with_rel_path(&vault.notes, "index.md"));
}

#[test]
fn vault_does_not_contain_notes_of_ignored_files() {
    let base_path = PathBuf::from("tests/fixtures/ignore");
    let vault = Vault::new(&base_path);

    assert_eq!(vault.base_path, base_path);
    assert_eq!(vault.notes.len(), 1);

    assert_eq!(
        is_there_note_with_rel_path(&vault.notes, "foo/bar.md"),
        false
    );
    assert!(is_there_note_with_rel_path(&vault.notes, "index.md"));
}
