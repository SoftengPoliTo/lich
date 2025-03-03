use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use toml::from_str;

use crate::output::ReportFormat;

// `[valgrind]` section options.
#[derive(Deserialize)]
pub(crate) struct Valgrind {
    enabled: bool,
}

impl Default for Valgrind {
    fn default() -> Self {
        Self { enabled: true }
    }
}

// `[powerstat]` section options.
#[derive(Deserialize)]
pub(crate) struct PowerStat {
    enabled: bool,
}

impl Default for PowerStat {
    fn default() -> Self {
        Self { enabled: true }
    }
}

// `[powertop]` section options.
#[derive(Deserialize)]
pub(crate) struct PowerTop {
    enabled: bool,
}

impl Default for PowerTop {
    fn default() -> Self {
        Self { enabled: true }
    }
}

// Accepted toml file structure.
#[derive(Deserialize)]
pub(crate) struct Configuration {
    pub(crate) report_path: PathBuf,
    #[serde(default)]
    pub(crate) format: ReportFormat,
    #[serde(default)]
    pub(crate) valgrind: Valgrind,
    #[serde(default)]
    pub(crate) powerstat: PowerStat,
    #[serde(default)]
    pub(crate) powertop: PowerTop,
}

impl Configuration {
    pub(crate) fn read(configuration_path: &Path) -> Self {
        let contents = read_to_string(configuration_path).unwrap();
        let configuration: Self = from_str(&contents).unwrap();

        assert!(
            !configuration.report_path.is_dir(),
            "The report path must be a directory!"
        );

        configuration
    }

    // If [`valgrind`] section does not exist, enable valgrind with the default
    // parameters.
    pub(crate) fn is_valgrind_enabled(&self) -> bool {
        self.valgrind.enabled
    }

    // If [`powerstat`] section does not exist, enable powerstat with the default
    // parameters.
    pub(crate) fn is_powerstat_enabled(&self) -> bool {
        self.powerstat.enabled
    }

    // If [`powertop`] section does not exist, enable powertop with the default
    // parameters.
    pub(crate) fn is_powertop_enabled(&self) -> bool {
        self.powertop.enabled
    }
}
