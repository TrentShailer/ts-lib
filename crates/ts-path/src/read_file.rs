use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::DisplayPath;

/// Error variants for reading a file.
#[derive(Debug)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ReadFileError {
    #[non_exhaustive]
    DoesNotExist { path: PathBuf },

    #[non_exhaustive]
    NotAFile { path: PathBuf },

    #[non_exhaustive]
    ReadError { path: PathBuf, source: io::Error },
}
impl core::fmt::Display for ReadFileError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            Self::DoesNotExist { path, .. } => {
                write!(f, "`{}` does not exist", path.opinionated_display())
            }
            Self::NotAFile { path, .. } => {
                write!(f, "`{}` is not a file", path.opinionated_display())
            }
            Self::ReadError { path, .. } => {
                write!(f, "could not read `{}`", path.opinionated_display())
            }
        }
    }
}
impl core::error::Error for ReadFileError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match &self {
            Self::ReadError { source, .. } => Some(source),
            _ => None,
        }
    }
}
impl ReadFileError {
    #[allow(missing_docs)]
    pub(crate) fn read_error(source: io::Error, path: &Path) -> Self {
        Self::ReadError {
            path: path.to_path_buf(),
            source,
        }
    }
}

/// Read a file, returning presentable error variants.
pub fn read_file(path: &Path) -> Result<Vec<u8>, ReadFileError> {
    if !fs::exists(path).map_err(|source| ReadFileError::read_error(source, path))? {
        return Err(ReadFileError::DoesNotExist {
            path: path.to_path_buf(),
        });
    }

    let metadata = path
        .metadata()
        .map_err(|source| ReadFileError::read_error(source, path))?;

    if metadata.is_dir() {
        return Err(ReadFileError::NotAFile {
            path: path.to_path_buf(),
        });
    }

    fs::read(path).map_err(|source| ReadFileError::read_error(source, path))
}

/// Read a file to a string, returning presentable error variants.
pub fn read_file_to_string(path: &Path) -> Result<String, ReadFileError> {
    if !fs::exists(path).map_err(|source| ReadFileError::read_error(source, path))? {
        return Err(ReadFileError::DoesNotExist {
            path: path.to_path_buf(),
        });
    }

    let metadata = path
        .metadata()
        .map_err(|source| ReadFileError::read_error(source, path))?;

    if metadata.is_dir() {
        return Err(ReadFileError::NotAFile {
            path: path.to_path_buf(),
        });
    }

    fs::read_to_string(path).map_err(|source| ReadFileError::read_error(source, path))
}
