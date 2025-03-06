use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use toml::from_str;

use crate::output::ReportFormat;
use crate::tools::{Args, PowerstatConfig, PowertopConfig, ValgrindConfig};

pub(crate) fn always_true() -> bool {
    true
}

// `[binary]` section options.
#[derive(Default, Deserialize)]
pub(crate) struct BinaryConfig {
    #[serde(default = "Vec::new")]
    pub(crate) args: Vec<String>,
}

impl Args for BinaryConfig {
    fn args(&self) -> &[String] {
        &self.args
    }
}

// Accepted toml file structure.
#[derive(Deserialize)]
pub(crate) struct Configurator {
    #[serde(rename = "binary-path")]
    pub(crate) binary_path: PathBuf,
    #[serde(rename = "report-path")]
    pub(crate) report_path: PathBuf,
    #[serde(default)]
    pub(crate) root: String,
    #[serde(default)]
    pub(crate) format: ReportFormat,
    #[serde(default)]
    pub(crate) binary: BinaryConfig,
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

        // TODO: If report path is a dir, create the default file `lich.extension`.

        // Binary path must not be a directory.
        assert!(
            !configuration.binary_path.is_dir(),
            "The binary path must not be a directory. Insert a path to the binary."
        );

        // TODO: Check whether binary path exists.

        configuration
    }

    // If [`valgrind`] section does not exist, enable valgrind with the default
    // parameters.
    pub(crate) fn is_valgrind_enabled(&self) -> bool {
        self.valgrind.enable
    }

    // If [`powerstat`] section does not exist, enable powerstat with the default
    // parameters.
    pub(crate) fn is_powerstat_enabled(&self) -> bool {
        self.powerstat.enable
    }

    // If [`powertop`] section does not exist, enable powertop with the default
    // parameters.
    pub(crate) fn is_powertop_enabled(&self) -> bool {
        self.powertop.enable
    }
}
