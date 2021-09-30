use std::path::Path;

use crate::file::File;

/// A `Note` entry
#[derive(Debug, PartialEq, Clone)]
pub struct Note {
    pub file: File,
}

impl Note {
    /// Creates a new `Note` from the given base and file paths
    pub fn new(base: &Path, file: &Path) -> Self {
        Self {
            file: File::new(base, file),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn constructs_a_note_from_paths() {
        let note = Note::new(&PathBuf::from("/foo/"), &PathBuf::from("/foo/index.md"));
        assert_eq!(
            note,
            Note {
                file: File::new(&PathBuf::from("/foo/"), &PathBuf::from("/foo/index.md")),
            }
        );
    }
}
