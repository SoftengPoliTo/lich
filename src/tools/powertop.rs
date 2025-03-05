use std::path::Path;

use serde::Deserialize;

use crate::configurator::BinaryConfig;

use super::{run_command, stderr_output, stdout_output, Args, ToolResult};

// `[powertop]` section options.
#[derive(Deserialize)]
pub(crate) struct PowertopConfig {
    pub(crate) enabled: bool,
    pub(crate) args: Vec<String>,
}

impl Default for PowertopConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            args: Vec::new(),
        }
    }
}

impl Args for PowertopConfig {
    fn args(&self) -> &[String] {
        &self.args
    }
}

pub(crate) struct Powertop;

impl Powertop {
    pub(crate) fn run(
        powertop_config: &PowertopConfig,
        binary_path: &Path,
        binary_config: &BinaryConfig,
    ) -> ToolResult {
        let powertop_output = run_command("powertop", powertop_config, binary_path, binary_config);

        let (body, result) = if powertop_output.status.success() {
            stdout_output(powertop_output.stdout)
        } else {
            stderr_output(powertop_output.stderr)
        };

        ToolResult {
            header: "Powertop",
            body,
            result,
        }
    }
}
