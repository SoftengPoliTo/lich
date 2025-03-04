use serde::Deserialize;

use super::{format_command_output, run_command, Args, ToolResult};

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
    pub(crate) fn run(powertop_config: &PowertopConfig) -> ToolResult {
        let powertop_output = run_command("powertop", powertop_config);

        let (body, result) = if powertop_output.status.success() {
            format_command_output(powertop_output.stdout)
        } else {
            format_command_output(powertop_output.stderr)
        };

        ToolResult {
            header: "Powertop",
            body,
            result,
        }
    }
}
