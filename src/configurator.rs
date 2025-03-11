use std::fs::{create_dir_all, read_to_string};
use std::path::{Path, PathBuf};

use serde::Deserialize;

use toml::from_str;

use crate::output::ReportFormat;
use crate::tools::{Args, PerfConfig, PowerstatConfig, PowertopConfig, ValgrindConfig};

const DOCKER_VOLUME_DIRECTORY: &str = "/lich";

fn create_docker_path(path: PathBuf) -> PathBuf {
    Path::new(DOCKER_VOLUME_DIRECTORY).join(path)
}

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
    pub(crate) docker: bool,
    #[serde(default)]
    pub(crate) format: ReportFormat,
    #[serde(default)]
    pub(crate) binary: BinaryConfig,
    #[serde(default)]
    pub(crate) valgrind: ValgrindConfig,
    #[serde(default)]
    pub(crate) perf: PerfConfig,
    #[serde(default)]
    pub(crate) powerstat: PowerstatConfig,
    #[serde(default)]
    pub(crate) powertop: PowertopConfig,
}

impl Configurator {
    pub(crate) fn read(configuration_path: &Path) -> Self {
        let contents = read_to_string(configuration_path).unwrap();
        let mut configuration: Self = from_str(&contents).unwrap();

        // If a docker run is requested, add the `/lich` directory to binary
        // and report path.
        if configuration.docker {
            configuration.binary_path = create_docker_path(configuration.binary_path);
            configuration.report_path = create_docker_path(configuration.report_path);
        }

        #[cfg(feature = "tracing")]
        {
            tracing::info!("Binary path: {:?}", configuration.binary_path);
            tracing::info!("Report path: {:?}", configuration.report_path);
        }

        // Binary path must not be a directory.
        assert!(
            configuration.binary_path.is_file(),
            "The binary path must not be a directory. Insert a path to the binary."
        );

        // Check binary path existence.
        match configuration.binary_path.try_exists() {
            Ok(true) => {}
            Ok(false) => panic!("The binary path does not exist"),
            Err(e) => panic!("Error checking the binary path existence: {e}"),
        }

        // Report path must be a directory.
        assert!(
            !configuration.report_path.is_file(),
            "The configuration report path must be a directory."
        );

        // Powertop configuration.
        Self::powertop(&mut configuration);

        // Create report path directory.
        create_dir_all(&configuration.report_path).unwrap();

        configuration
    }

    // If [`valgrind`] section does not exist, enable valgrind with the default
    // parameters.
    pub(crate) fn is_valgrind_enabled(&self) -> bool {
        self.valgrind.enable
    }

    // If [`perf`] section does not exist, enable perf with the default
    // parameters.
    pub(crate) fn is_perf_enabled(&self) -> bool {
        self.perf.enable
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

    // Powertop configuration.
    fn powertop(config: &mut Configurator) {
        // Check powertop csv path.
        assert!(
            config.powertop.check_csv_output_path(),
            "The csv output path must be a file path with the `csv` extension."
        );

        // Add `csv` output to powertop arguments.
        config.powertop.add_csv_output_to_args();
    }
}
