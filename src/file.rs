use std::path::{Path, PathBuf};

use crate::error::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct File {
    file_path: PathBuf,
    base_path: PathBuf,
}

impl File {
    /// Creates a new `File` with the given base and file `Path`s.
    pub fn new(base: &Path, file: &Path) -> Self {
        Self {
            base_path: base.to_path_buf(),
            file_path: file.to_path_buf(),
        }
    }
    /// Returns the absolute path of the file
    pub fn abs_path(&self) -> &Path {
        self.file_path.as_path()
    }

    /// Returns the relative path of the file to its base path
    pub fn rel_path(&self) -> Result<&Path, Error> {
        self.abs_path().strip_prefix(&self.base_path).map_err(|_| {
            Error::InvalidBasePath(self.base_path.to_path_buf(), self.file_path.to_path_buf())
        })
    }
}

pub fn has_markdown_extension(path: &Path) -> bool {
    match path.extension() {
        Some(extension) => extension == "md",
        None => false,
    }
}

pub fn walk(path: &Path, filter: fn(&Path) -> bool) -> Vec<PathBuf> {
    let walker = ignore::Walk::new(path);
    walker
        .into_iter()
        .filter_map(|result| result.ok())
        .filter_map(|entry| {
            if entry.file_type()?.is_file() {
                Some(entry.path().to_path_buf())
            } else {
                None
            }
        })
        .filter(|entry_path| filter(entry_path))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_a_file_from_paths() {
        let file = File::new(&PathBuf::from("/foo/"), &PathBuf::from("/foo/index.md"));
        assert_eq!(
            file,
            File {
                base_path: PathBuf::from("/foo/"),
                file_path: PathBuf::from("/foo/index.md"),
            }
        );
    }

    #[test]
    fn abs_path_returns_the_full_path() {
        let file = File::new(
            &PathBuf::from("/foo/bar/"),
            &PathBuf::from("/foo/bar/index.md"),
        );
        assert_eq!(file.abs_path(), &PathBuf::from("/foo/bar/index.md"));
    }

    #[test]
    fn rel_path_returns_the_relative_path() {
        let file = File::new(
            &PathBuf::from("/foo/bar/"),
            &PathBuf::from("/foo/bar/index.md"),
        );
        assert_eq!(file.rel_path(), Ok(PathBuf::from("index.md").as_path()));
    }

    #[test]
    fn rel_path_returns_error_if_base_path_is_incorrect() {
        let file = File::new(
            &PathBuf::from("/lorem/"),
            &PathBuf::from("/foo/bar/index.md"),
        );

        assert_eq!(
            file.rel_path(),
            Err(Error::InvalidBasePath(
                PathBuf::from("/lorem/"),
                PathBuf::from("/foo/bar/index.md")
            ))
        );
    }
}
