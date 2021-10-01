use std::path::{Path, PathBuf};

use crate::asset::Asset;
use crate::error::Error;
use crate::file;
use crate::note::Note;

/// A `Vault` containing all the entries in the file tree.
#[derive(Debug, PartialEq)]
pub struct Vault {
    pub base_path: PathBuf,
    pub notes: Vec<Note>,
    pub assets: Vec<Asset>,
}

impl Vault {
    /// Creates a new `Vault` from a given base Path. Yields error if the
    /// path does not exist.
    pub fn open(base: &Path) -> Result<Self, Error> {
        if !base.is_dir() {
            return Err(Error::PathDoesNotExist(base.to_path_buf()));
        }

        Ok(Self {
            base_path: base.to_path_buf(),
            notes: file::walk(base, file::has_markdown_extension)
                .into_iter()
                .map(|entry_path| Note::new(base, &entry_path))
                .collect(),
            assets: file::walk(base, |entry_path| !file::has_markdown_extension(entry_path))
                .into_iter()
                .map(|entry_path| Asset::new(base, &entry_path))
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;
    use assert_fs::TempDir;

    #[test]
    fn returns_error_when_opening_a_non_existing_dir() {
        let path = PathBuf::from("lorem");
        let vault_result = Vault::open(&path);
        assert_eq!(vault_result, Err(Error::PathDoesNotExist(path)));
    }

    #[test]
    fn opens_a_vault_from_empty_dir() {
        let temp_dir = TempDir::new().expect("Could not create temporary dir");
        let vault = Vault::open(temp_dir.path()).unwrap();

        assert_eq!(
            vault,
            Vault {
                base_path: temp_dir.path().to_path_buf(),
                assets: vec![],
                notes: vec![],
            }
        );

        temp_dir.close().expect("Could not close temporary dir");
    }

    #[test]
    fn opens_a_vault_from_non_empty_dir() {
        let temp_dir = TempDir::new().expect("Could not create temporary dir");
        temp_dir
            .child("foo.md")
            .touch()
            .expect("Could not create temporary file");
        temp_dir
            .child("lorem/ipsum.md")
            .touch()
            .expect("Could not create temporary file");
        temp_dir
            .child("image.svg")
            .touch()
            .expect("Could not create temporary file");

        let vault = Vault::open(temp_dir.path()).unwrap();

        assert_eq!(vault.base_path, temp_dir.path().to_path_buf());
        assert_eq!(vault.notes.len(), 2);
        assert_eq!(vault.assets.len(), 1);

        temp_dir.close().expect("Could not close temporary dir");
    }
}
