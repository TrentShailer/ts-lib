//! CLI subcommands for config files

use std::{fs, io::stdin, process};

use argh::FromArgs;
use ts_ansi::format_success;
use ts_error::ProgramReport;
use ts_path::DisplayPath;

use crate::{ConfigFile, try_load};

#[derive(FromArgs, Debug, PartialEq)]
#[argh(
    subcommand,
    name = "config",
    description = "Manage application config."
)]
/// Manage application config.
///
/// ## Usage
/// ```
/// #[derive(argh::FromArgs, PartialEq, Debug)]
/// struct Cli {
///     #[argh(subcommand)]
///     subcommand: Option<Subcommand>,
/// }
///
/// #[derive(argh::FromArgs, Debug, PartialEq)]
/// #[argh(subcommand)]
/// enum Subcommand {
///     Config(ts_config::cli::ConfigCommand)
/// }
///
/// #[derive(Default, Debug, serde::Serialize, serde::Deserialize, ts_config::schemars::JsonSchema)]
/// struct Config {
///     field_a: usize
/// }
/// impl ts_config::ConfigFile for Config {
///     fn config_file_path() -> std::path::PathBuf {
///         std::path::PathBuf::from("./config.json")
///     }
/// }
///
/// let cli: Cli = argh::from_env();
/// if let Some(subcommand) = cli.subcommand.as_ref() {
///     match subcommand {
///         Subcommand::Config(config_command) => config_command.execute::<Config>(),
///     }
/// }
/// ```
pub struct ConfigCommand {
    /// the config subcommand
    #[argh(subcommand)]
    subcommand: ConfigSubcommand,
}
impl ConfigCommand {
    /// Executes the config command.
    pub fn execute<C: ConfigFile>(&self) -> ! {
        match &self.subcommand {
            ConfigSubcommand::Lint(lint_subcommand) => lint_subcommand.execute::<C>(),
            ConfigSubcommand::Init(init_subcommand) => init_subcommand.execute::<C>(),
        }
    }
}

#[derive(FromArgs, Debug, PartialEq)]
#[argh(subcommand)]
#[non_exhaustive]
/// Manage application config.
pub enum ConfigSubcommand {
    /// Lint the config file.
    Lint(LintSubcommand),
    /// Initialise a default config file.    
    Init(InitSubcommand),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "lint")]
#[non_exhaustive]
/// Lint the config file.
pub struct LintSubcommand {}
impl LintSubcommand {
    /// Lints the config, exits the application on success, or failure.
    pub fn execute<C: ConfigFile>(&self) -> ! {
        let exit_code = match try_load::<C>() {
            Ok(_) => {
                eprintln!("{}", format_success!("config file is valid"));
                0
            }
            Err(error) => {
                let report = ProgramReport::from(error);
                eprintln!("{report}");
                1
            }
        };

        process::exit(exit_code)
    }
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "init")]
#[non_exhaustive]
/// Initialise a default config file.
pub struct InitSubcommand {
    #[argh(switch)]
    /// forcefully overwrite any existing config file
    force: bool,
}
impl InitSubcommand {
    /// Initialise the config, exits the application on success, or failure.
    pub fn execute<C: ConfigFile>(&self) -> ! {
        match fs::exists(C::config_file_path()) {
            Ok(exists) => {
                if exists && !self.force {
                    eprint!(
                        "A config file already exists at ({}), overwrite it (y/n): ",
                        C::config_file_path().opinionated_display()
                    );
                    let mut buffer = String::new();
                    if let Err(error) = stdin().read_line(&mut buffer) {
                        let report = ProgramReport::from(error);
                        eprintln!("{report}");
                        process::exit(1);
                    };

                    if buffer.trim_end() != "y" {
                        process::exit(1);
                    }
                }
            }
            Err(error) => {
                let report = ProgramReport::from(error);
                eprintln!("{report}");
                process::exit(1)
            }
        }

        if let Err(error) = C::default().write() {
            let report = ProgramReport::from(error);
            eprintln!("{report}");
            process::exit(1)
        };

        eprintln!(
            "{}",
            format_success!(
                "initialised default config at {}",
                C::config_file_path().opinionated_display()
            )
        );
        process::exit(0)
    }
}
