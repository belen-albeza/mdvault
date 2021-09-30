use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("entry at path `{0}` is not a note")]
    NotANote(PathBuf),
    #[error("invalid base path `{0}` for file `{1}`")]
    InvalidBasePath(PathBuf, PathBuf),
}
