use std::path::Path;

use serde::Deserialize;

use crate::configurator::{always_true, BinaryConfig};

use super::{run_command, stderr_output, stdout_output, Args, ToolResult};

// `[PowerStat]` section options.
#[derive(Deserialize)]
pub(crate) struct PowerstatConfig {
    #[serde(default = "always_true")]
    pub(crate) enabled: bool,
    #[serde(default = "Vec::new")]
    pub(crate) args: Vec<String>,
}

impl Default for PowerstatConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            args: Vec::new(),
        }
    }
}

impl Args for PowerstatConfig {
    fn args(&self) -> &[String] {
        &self.args
    }
}

pub(crate) struct Powerstat;

impl Powerstat {
    pub(crate) fn run(
        powerstat_config: &PowerstatConfig,
        binary_path: &Path,
        binary_config: &BinaryConfig,
    ) -> ToolResult {
        let powerstat_output =
            run_command("powerstat", powerstat_config, binary_path, binary_config);

        let (body, result) = if powerstat_output.status.success() {
            stdout_output(powerstat_output.stdout)
        } else {
            stderr_output(powerstat_output.stderr)
        };

        ToolResult {
            header: "Powerstat",
            body,
            result,
        }
    }
}
