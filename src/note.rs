use std::path::{Path, PathBuf};

/// A `Note` representing a file within the `Vault` file tree.
#[derive(Debug, PartialEq)]
pub struct Note {
    file_path: PathBuf,
    base_path: PathBuf,
}

impl Note {
    /// Creates a new `Note` with the given base and file `Path`s.
    pub fn new(base: &Path, file: &Path) -> Self {
        Self {
            base_path: base.to_path_buf(),
            file_path: file.to_path_buf(),
        }
    }

    /// Returns the absolute path of the note
    pub fn abs_path(&self) -> &Path {
        self.file_path.as_path()
    }

    /// Returns the relative path of the note
    pub fn rel_path(&self) -> &Path {
        self.abs_path()
            .strip_prefix(&self.base_path)
            .expect("File should be under the base path")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_a_note_from_paths() {
        let note = Note::new(&PathBuf::from("/foo/"), &PathBuf::from("/foo/index.md"));
        assert_eq!(
            note,
            Note {
                base_path: PathBuf::from("/foo/"),
                file_path: PathBuf::from("/foo/index.md")
            }
        );
    }

    #[test]
    fn abs_path_returns_the_full_path() {
        let note = Note {
            file_path: PathBuf::from("/foo/bar/index.md"),
            base_path: PathBuf::from("/foo/bar/"),
        };
        assert_eq!(note.abs_path(), &PathBuf::from("/foo/bar/index.md"));
    }

    #[test]
    fn rel_path_returns_the_relative_path() {
        let note = Note {
            file_path: PathBuf::from("/foo/bar/index.md"),
            base_path: PathBuf::from("/foo/bar/"),
        };
        assert_eq!(note.rel_path(), &PathBuf::from("index.md"));
    }

    #[test]
    #[should_panic]
    fn rel_path_panics_if_base_path_is_incorrect() {
        let note = Note {
            file_path: PathBuf::from("/foo/bar/index.md"),
            base_path: PathBuf::from("/lorem/"),
        };

        note.rel_path();
    }
}
