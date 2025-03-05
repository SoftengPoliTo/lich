use std::path::Path;

use serde::Deserialize;

use crate::configurator::{always_true, BinaryConfig};

use super::{
    check_tool_existence, run_tool_with_input, stderr_output, stdout_output,
    sudo_run_tool_with_input, Args, ToolResult,
};

// `[powertop]` section options.
#[derive(Deserialize)]
pub(crate) struct PowertopConfig {
    #[serde(default = "always_true")]
    pub(crate) enabled: bool,
    #[serde(default = "Vec::new")]
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
    pub(crate) fn check_existence() {
        check_tool_existence("powertop").expect("powertop cannot be found on the system.");
    }

    pub(crate) fn run(
        root: &str,
        powertop_config: &PowertopConfig,
        binary_path: &Path,
        binary_config: &BinaryConfig,
    ) -> ToolResult {
        let binary_input = Self::create_binary_input(binary_path, binary_config.args());

        let powertop_output = if root.is_empty() {
            run_tool_with_input("powertop", powertop_config, binary_input)
        } else {
            sudo_run_tool_with_input("powertop", powertop_config, binary_input, root)
        };

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

    fn create_binary_input(binary_path: &Path, binary_arguments: &[String]) -> String {
        let arguments = binary_arguments.join(" ");
        format!("{} {arguments}", binary_path.to_str().unwrap())
    }
}
