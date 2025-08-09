use schemars::{SchemaGenerator, generate::SchemaSettings};
use ts_error::diagnostic::Diagnostics;
use ts_io::{ReadFileError, read_file_to_string};
use ts_json::{ValidationError, validate};

use crate::ConfigFile;

/// Error variants for loading config.
#[derive(Debug)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum LoadConfigError {
    #[non_exhaustive]
    SerailizeSchema { source: serde_json::Error },

    #[non_exhaustive]
    ValidationFailure { source: ValidationError },

    #[non_exhaustive]
    InvalidConfig { source: Diagnostics },

    #[non_exhaustive]
    DeserializeConfig { source: serde_json::Error },

    #[non_exhaustive]
    ReadConfig { source: ReadFileError },
}
impl core::fmt::Display for LoadConfigError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            Self::SerailizeSchema { .. } => {
                write!(f, "JSON schema for the config could not be serialized")
            }
            Self::ValidationFailure { .. } => write!(f, "could not validate config file"),
            Self::InvalidConfig { .. } => write!(f, "config file is invalid"),
            Self::DeserializeConfig { .. } => write!(f, "config file could not be deserialized"),
            Self::ReadConfig { .. } => write!(f, "could not read config file"),
        }
    }
}
impl core::error::Error for LoadConfigError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match &self {
            Self::DeserializeConfig { source, .. } | Self::SerailizeSchema { source, .. } => {
                Some(source)
            }
            Self::ValidationFailure { source, .. } => Some(source),
            Self::InvalidConfig { source, .. } => Some(source),
            Self::ReadConfig { source, .. } => Some(source),
        }
    }
}

/// Try load a config file, linting it against its JSON schema.
pub fn try_load<C: ConfigFile>() -> Result<C, LoadConfigError> {
    let source = read_file_to_string(&C::config_file_path())
        .map_err(|source| LoadConfigError::ReadConfig { source })?;

    let schema_generator = SchemaGenerator::from(SchemaSettings::draft07());
    let schema = schema_generator.into_root_schema_for::<C>();
    let schema = serde_json::to_string(&schema)
        .map_err(|source| LoadConfigError::SerailizeSchema { source })?;

    let diagnostics = validate(&source, &schema, Some(C::config_file_path()).as_deref())
        .map_err(|source| LoadConfigError::ValidationFailure { source })?;

    if !diagnostics.is_empty() {
        Err(LoadConfigError::InvalidConfig {
            source: diagnostics,
        })
    } else {
        serde_json::from_str(&source)
            .map_err(|source| LoadConfigError::DeserializeConfig { source })
    }
}
