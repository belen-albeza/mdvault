use mdvault::Asset;
use mdvault::Note;
use mdvault::Vault;

use std::path::PathBuf;

fn is_there_note_with_rel_path(notes: &[Note], path_str: &str) -> bool {
    let path = PathBuf::from(path_str);
    notes
        .iter()
        .any(|entry| entry.file.rel_path().unwrap() == path)
}

fn is_there_asset_with_rel_path(assets: &[Asset], path_str: &str) -> bool {
    let path = PathBuf::from(path_str);
    assets
        .iter()
        .any(|entry| entry.file.rel_path().unwrap() == path)
}

#[test]
fn vault_scans_file_tree_on_creation() {
    let base_path = PathBuf::from("tests/fixtures/basic");
    let vault = Vault::open(&base_path).unwrap();

    assert_eq!(vault.base_path, base_path);
    assert_eq!(vault.notes.len(), 2);
    assert_eq!(vault.assets.len(), 1);

    assert!(is_there_note_with_rel_path(&vault.notes, "foo/bar.md"));
    assert!(is_there_note_with_rel_path(&vault.notes, "index.md"));
    assert!(is_there_asset_with_rel_path(&vault.assets, "image.svg"));
}

#[test]
fn vault_does_not_contain_notes_of_ignored_files() {
    let base_path = PathBuf::from("tests/fixtures/ignore");
    let vault = Vault::open(&base_path).unwrap();

    assert_eq!(vault.base_path, base_path);
    assert_eq!(vault.notes.len(), 1);

    assert!(!is_there_note_with_rel_path(&vault.notes, "foo/bar.md"));
    assert!(is_there_note_with_rel_path(&vault.notes, "index.md"));
}
