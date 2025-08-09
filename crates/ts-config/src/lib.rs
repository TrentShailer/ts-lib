//! # `ts-config`
//!
//! Helpers for application config.

mod load;

use std::{fs, io, path::PathBuf};

use schemars::JsonSchema;
use serde::{Serialize, de::DeserializeOwned};

pub use load::{LoadConfigError, try_load};

/// Trait defining a struct as representing a config file.
pub trait ConfigFile: Default + DeserializeOwned + Serialize + JsonSchema {
    /// The path to the config file.
    fn config_file_path() -> PathBuf;

    /// Delete the config file.
    fn delete(&self) -> io::Result<()> {
        fs::remove_file(Self::config_file_path())
    }

    /// Write the config file.
    fn write(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self).map_err(io::Error::other)?;
        fs::write(Self::config_file_path(), json)
    }
}
