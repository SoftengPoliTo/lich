use std::path::Path;

use serde::Deserialize;

use crate::configurator::BinaryConfig;

use super::{format_command_output, run_command, Args, ToolResult};

// `[valgrind]` section options.
#[derive(Deserialize)]
pub(crate) struct ValgrindConfig {
    pub(crate) enabled: bool,
    pub(crate) args: Vec<String>,
}

impl Default for ValgrindConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            args: Vec::new(),
        }
    }
}

impl Args for ValgrindConfig {
    fn args(&self) -> &[String] {
        &self.args
    }
}

pub(crate) struct Valgrind;

impl Valgrind {
    pub(crate) fn run(
        valgrind_config: &ValgrindConfig,
        binary_path: &Path,
        binary_config: &BinaryConfig,
    ) -> ToolResult {
        // Run valgrind command.
        let valgrind_output = run_command("valgrind", valgrind_config, binary_path, binary_config);

        let (body, result) = if valgrind_output.status.success() {
            format_command_output(valgrind_output.stdout)
        } else {
            format_command_output(valgrind_output.stderr)
        };

        ToolResult {
            header: "Valgrind",
            body,
            result,
        }
    }
}
