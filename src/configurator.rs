use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use toml::from_str;

use crate::output::ReportFormat;
use crate::tools::{PowerstatConfig, PowertopConfig, ValgrindConfig};

// Accepted toml file structure.
#[derive(Deserialize)]
pub(crate) struct Configurator {
    pub(crate) report_path: PathBuf,
    #[serde(default)]
    pub(crate) format: ReportFormat,
    #[serde(default)]
    pub(crate) valgrind: ValgrindConfig,
    #[serde(default)]
    pub(crate) powerstat: PowerstatConfig,
    #[serde(default)]
    pub(crate) powertop: PowertopConfig,
}

impl Configurator {
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
