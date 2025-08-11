//! A simple normalization of a path.

use core::slice;
use std::{
    ffi::{OsStr, OsString},
    path::{Component, MAIN_SEPARATOR_STR, Path, PathBuf, Prefix},
};

/// Custom component as [`std::path::Component`] is difficult to construct.
#[derive(Debug)]
#[allow(clippy::missing_docs_in_private_items)]
enum CustomComponent<'a> {
    Prefix(OsString),
    CurDir,
    ParentDir,
    RootDir,
    Normal(&'a OsStr),
}
impl<'a> CustomComponent<'a> {
    /// Convert the component to an [`OsStr`]
    pub fn as_os_str(&'a self) -> &'a OsStr {
        match self {
            Self::Prefix(p) => p.as_os_str(),
            Self::RootDir => OsStr::new(MAIN_SEPARATOR_STR),
            Self::CurDir => OsStr::new("."),
            Self::ParentDir => OsStr::new(".."),
            Self::Normal(path) => path,
        }
    }
}
impl<'a> From<Component<'a>> for CustomComponent<'a> {
    fn from(value: Component<'a>) -> Self {
        match value {
            Component::Prefix(prefix_component) => match prefix_component.kind() {
                Prefix::Verbatim(os_str) => Self::Normal(os_str),
                Prefix::DeviceNS(os_str) => {
                    let mut prefix = OsString::with_capacity(4 + os_str.len());
                    prefix.push(r"\\.\");
                    prefix.push(os_str);
                    Self::Prefix(prefix)
                }
                Prefix::VerbatimUNC(server, share) | Prefix::UNC(server, share) => {
                    let mut prefix = OsString::with_capacity(2 + server.len() + share.len());
                    prefix.push(r"\\");
                    prefix.push(server);
                    prefix.push(r"\");
                    prefix.push(share);
                    Self::Prefix(prefix)
                }
                Prefix::VerbatimDisk(disk) | Prefix::Disk(disk) => {
                    let mut prefix = OsString::with_capacity(2);
                    let letter = str::from_utf8(slice::from_ref(&disk)).unwrap_or("C");
                    prefix.push(letter);
                    prefix.push(":");
                    Self::Prefix(prefix)
                }
            },
            Component::RootDir => Self::RootDir,
            Component::CurDir => Self::CurDir,
            Component::ParentDir => Self::ParentDir,
            Component::Normal(os_str) => Self::Normal(os_str),
        }
    }
}
impl AsRef<OsStr> for CustomComponent<'_> {
    #[inline]
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}
impl AsRef<Path> for CustomComponent<'_> {
    #[inline]
    fn as_ref(&self) -> &Path {
        self.as_os_str().as_ref()
    }
}

/// Extension trait to call [`crate::normalize_path`] on a path.
pub trait NormalizePath {
    /// Normalize a path using only the components of the path.
    ///
    /// This will ignore any symbolic links, and strip the verbatim `\\?\` prefixes, so should only be
    /// used when that can be tolerated.
    fn normalized(&self) -> PathBuf;
}
impl<P: AsRef<Path>> NormalizePath for P {
    fn normalized(&self) -> PathBuf {
        normalize_path(self.as_ref())
    }
}

/// Normalize a path using only the components of the path.
///
/// This will ignore any symbolic links, and strip the verbatim `\\?\` prefixes, so should only be
/// used when that can be tolerated.
pub fn normalize_path(path: &Path) -> PathBuf {
    let mut output: Vec<CustomComponent> = Vec::with_capacity(path.components().count());

    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if let Some(component) = output.last()
                    && matches!(component, CustomComponent::Normal(_))
                {
                    output.pop();
                } else {
                    output.push(component.into());
                }
            }
            Component::RootDir => {
                if output
                    .last()
                    .is_none_or(|component| matches!(component, CustomComponent::Prefix(_)))
                {
                    output.push(component.into());
                };
            }
            _ => output.push(component.into()),
        }
    }

    PathBuf::from_iter(output)
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::NormalizePath;

    #[test]
    fn handles_verbatim_prefixes() {
        let expected = Path::new(r"some-verbatim-path\some-more-path");
        let data = Path::new(r"\\?\some-verbatim-path\some-more-path");
        assert_eq!(expected, data.normalized());

        let expected = Path::new(r"T:\some-verbatim-path\some-more-path");
        let data = Path::new(r"\\?\T:\some-verbatim-path\some-more-path");
        assert_eq!(expected, data.normalized());

        let expected = Path::new(r"\\server\share\some-more-path");
        let data = Path::new(r"\\?\UNC\server\share\some-more-path");
        assert_eq!(expected, data.normalized());
    }

    #[test]
    fn handles_prefixes() {
        let data = Path::new(r"\\server\share\some-more-path");
        assert_eq!(data, data.normalized());

        let data = Path::new(r"C:\path\some-more-path");
        assert_eq!(data, data.normalized());

        let data = Path::new(r"\\.\path\some-more-path");
        assert_eq!(data, data.normalized());
    }

    #[test]
    fn handles_parent() {
        let expected = Path::new(r"../../path");
        let data = Path::new(r"../../some-parent/../path");
        assert_eq!(expected, data.normalized());

        let expected = Path::new(r"../../../");
        let data = Path::new(r"../../some-parent/../../path/..");
        assert_eq!(expected, data.normalized());
    }

    #[test]
    fn handles_current_dir() {
        let expected = Path::new(r"some/annoying/path");
        let data = Path::new(r"./some/./././annoying/path/.");
        assert_eq!(expected, data.normalized());
    }
}
