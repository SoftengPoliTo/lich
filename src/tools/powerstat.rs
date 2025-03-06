use std::path::Path;

use serde::Deserialize;

use crate::configurator::{always_true, BinaryConfig};

use super::{
    check_tool_existence, run_tool, stderr_output, stdout_output, sudo_run_tool, Args, ToolResult,
};

// `[powerstat]` section options.
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
    pub(crate) fn check_existence() {
        check_tool_existence("powerstat").expect("powerstat cannot be found on the system.");
    }

    pub(crate) fn run(
        root: &str,
        powerstat_config: &PowerstatConfig,
        binary_path: &Path,
        binary_config: &BinaryConfig,
    ) -> ToolResult {
        let powerstat_output = if root.is_empty() {
            run_tool("powerstat", powerstat_config, binary_path, binary_config)
        } else {
            sudo_run_tool(
                "powerstat",
                powerstat_config,
                binary_path,
                binary_config,
                root,
            )
        };

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
