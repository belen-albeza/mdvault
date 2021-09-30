use ignore::Walk;
use std::path::{Path, PathBuf};

use crate::note::Note;

/// A `Vault` containing all the notes in the file tree.
#[derive(Debug, PartialEq)]
pub struct Vault {
    pub base_path: PathBuf,
    pub notes: Vec<Note>,
}

impl Vault {
    pub fn new(base: &Path) -> Self {
        let walker = Walk::new(base);
        Self {
            base_path: base.to_path_buf(),
            notes: walker
                .into_iter()
                .filter_map(|result| result.ok())
                .filter_map(|entry| {
                    if entry.file_type().unwrap().is_file() {
                        Some(Note::new(base, entry.path()))
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;
    use assert_fs::TempDir;

    #[test]
    fn creates_a_vault_from_empty_dir() {
        let temp_dir = TempDir::new().expect("Could not create temporary dir");
        let vault = Vault::new(temp_dir.path());

        assert_eq!(
            vault,
            Vault {
                notes: vec![],
                base_path: temp_dir.path().to_path_buf()
            }
        );

        temp_dir.close().expect("Could not close temporary dir");
    }

    #[test]
    fn creates_a_vault_from_non_empty_dir() {
        let temp_dir = TempDir::new().expect("Could not create temporary dir");
        temp_dir
            .child("foo.md")
            .touch()
            .expect("Could not create temporary file");
        temp_dir
            .child("lorem/ipsum.md")
            .touch()
            .expect("Could not create temporary file");

        let vault = Vault::new(temp_dir.path());

        assert_eq!(vault.base_path, temp_dir.path().to_path_buf());
        assert_eq!(vault.notes.len(), 2);

        temp_dir.close().expect("Could not close temporary dir");
    }
}
