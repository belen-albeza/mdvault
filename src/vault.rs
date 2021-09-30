use crate::note::Note;
use std::path::Path;

/// A `Vault` containing all the notes in the file tree.
#[derive(Debug, PartialEq)]
pub struct Vault {
    notes: Vec<Note>,
}

impl Vault {
    pub fn new(_source: &Path) -> Self {
        Self { notes: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::TempDir;

    #[test]
    fn creates_a_vault_from_empty_source_dir() {
        let source = TempDir::new().expect("Could not create temporary dir");
        let vault = Vault::new(&source);
        assert_eq!(vault, Vault { notes: vec![] })
    }
}
