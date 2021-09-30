use std::path::Path;

use crate::file::File;

/// An `Asset` entry
#[derive(Debug, PartialEq, Clone)]
pub struct Asset {
    pub file: File,
}

impl Asset {
    /// Creates a new `Asset` from the given base and file paths
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
    fn constructs_an_asset_from_paths() {
        let asset = Asset::new(&PathBuf::from("/foo/"), &PathBuf::from("/foo/image.svg"));
        assert_eq!(
            asset,
            Asset {
                file: File::new(&PathBuf::from("/foo/"), &PathBuf::from("/foo/image.svg")),
            }
        );
    }
}
