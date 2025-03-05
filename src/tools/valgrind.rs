use std::path::Path;

use serde::Deserialize;

use crate::configurator::{always_true, BinaryConfig};

use super::{check_tool_existence, run_tool, stderr_output, stdout_output, Args, ToolResult};

// `[valgrind]` section options.
#[derive(Deserialize)]
pub(crate) struct ValgrindConfig {
    #[serde(default = "always_true")]
    pub(crate) enabled: bool,
    #[serde(default = "Vec::new")]
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
    pub(crate) fn check_existence() {
        check_tool_existence("valgrind").expect("Valgrind cannot be found on the system.");
    }

    pub(crate) fn run(
        valgrind_config: &ValgrindConfig,
        binary_path: &Path,
        binary_config: &BinaryConfig,
    ) -> ToolResult {
        let valgrind_output = run_tool("valgrind", valgrind_config, binary_path, binary_config);

        let (body, result) = if valgrind_output.status.success() {
            stdout_output(valgrind_output.stdout)
        } else {
            stderr_output(valgrind_output.stderr)
        };

        ToolResult {
            header: "Valgrind",
            body,
            result,
        }
    }
}
