use core::iter::repeat_n;
use std::{
    fs,
    path::{Component, Path, PathBuf},
};

use crate::NormalizePath;

/// Extension trait to get the relative path.
pub trait RelativePath {
    /// Returns the path to navigate from a source path to self.
    fn relative_to(&self, source: &Path) -> PathBuf;
}

impl<P: AsRef<Path>> RelativePath for P {
    fn relative_to(&self, source: &Path) -> PathBuf {
        relative_path(source, self.as_ref())
    }
}

/// Returns the path to navigate from a source path to a target path.
pub fn relative_path(source: &Path, target: &Path) -> PathBuf {
    let source = fs::canonicalize(source)
        .unwrap_or_else(|_| source.to_path_buf())
        .normalized();
    let source: Vec<_> = source.components().collect();

    let target = fs::canonicalize(target)
        .unwrap_or_else(|_| target.to_path_buf())
        .normalized();
    let target: Vec<_> = target.components().collect();

    let diverge_index = {
        let mut index = 0;

        for source_component in source.iter() {
            let Some(target_component) = target.get(index) else {
                break;
            };

            if source_component != target_component {
                break;
            }

            index += 1;
        }

        index
    };

    let output_components: Vec<_> = repeat_n(&Component::ParentDir, source.len() - diverge_index)
        .chain(target.get(diverge_index..).unwrap_or_default())
        .collect();

    if output_components.is_empty() {
        PathBuf::from_iter(&[Component::CurDir])
    } else {
        PathBuf::from_iter(output_components)
    }
}

#[cfg(test)]
mod test {
    use std::path::{Path, PathBuf};

    use crate::relative::RelativePath;

    #[test]
    fn handles_relative() {
        let source = Path::new("/root/dir-a/dir-b");
        let target = Path::new("/root/dir-c/dir-d");
        assert_eq!(
            PathBuf::from("../../dir-c/dir-d"),
            target.relative_to(source)
        );

        let source = Path::new("dir-a/dir-b");
        let target = Path::new("dir-a/dir-b");
        assert_eq!(PathBuf::from("."), target.relative_to(source));

        let source = Path::new("../dir-a/dir-b");
        let target = Path::new("./dir-a/./dir-b");
        assert_eq!(
            PathBuf::from("../../../dir-a/dir-b"),
            target.relative_to(source)
        );
    }
}
