use alloc::borrow::Cow;
use std::path::{Component, Path};

use crate::NormalizePath;

/// Extension trait to display a path.
pub trait DisplayPath {
    /// Opinionated display for a path    
    fn opinionated_display(&self) -> String;
}

impl<P: AsRef<Path>> DisplayPath for P {
    fn opinionated_display(&self) -> String {
        display_path(self.as_ref())
    }
}

/// Opinionated display for a path:
/// * Normalises the path.
/// * Prefixed paths use the `\` separator, all other paths use the `/` separator.
pub fn display_path(path: &Path) -> String {
    let path = path.normalized();

    if path.as_path() == Path::new("") {
        return ".".to_string();
    }

    let has_prefix = path
        .components()
        .next()
        .is_some_and(|component| matches!(component, Component::Prefix(_)));

    let separator = if has_prefix { r"\" } else { "/" };

    path.components()
        .filter_map(|component| {
            if has_prefix && matches!(component, Component::RootDir) {
                None
            } else if matches!(component, Component::RootDir) {
                Some(Cow::Borrowed(""))
            } else {
                Some(component.as_os_str().to_string_lossy())
            }
        })
        .collect::<Vec<_>>()
        .join(separator)
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::display::DisplayPath;

    #[test]
    fn handles_relative() {
        let expected = "some/relative/path/1";
        let data = Path::new(r"some\relative\path\1");
        assert_eq!(expected, data.opinionated_display());

        let expected = "some/relative/path/2";
        let data = Path::new(r".\some\relative\path\2");
        assert_eq!(expected, data.opinionated_display());
    }

    #[test]
    fn handles_windows_style() {
        let expected = r"C:\some\absolute\path\1";
        let data = Path::new(r"C:\some\absolute\path\1");
        assert_eq!(expected, data.opinionated_display());

        let expected = r"C:\some\absolute\path\2";
        let data = Path::new(r"C:/some/absolute/path/2");
        assert_eq!(expected, data.opinionated_display());
    }

    #[test]
    fn handles_unix_style() {
        let expected = r"/some/absolute/path/1";
        let data = Path::new(r"/some/absolute/path/1");
        assert_eq!(expected, data.opinionated_display());

        let expected = r"/some/absolute/path/2";
        let data = Path::new(r"\some\absolute\path\2");
        assert_eq!(expected, data.opinionated_display());
    }
}
