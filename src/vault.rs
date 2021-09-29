use std::path::{Path, PathBuf};

/// A `Note` representing a file within the `Vault` file tree.
#[derive(Debug, PartialEq)]
pub struct Note {
    file_path: PathBuf,
    src_path: PathBuf,
}

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

    #[test]
    fn creates_a_vault_from_the_source_dir() {
        let vault = Vault::new(&PathBuf::from("."));
        assert_eq!(vault, Vault { notes: vec![] })
    }
}
