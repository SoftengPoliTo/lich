use serde::Deserialize;

use super::{format_command_output, run_command, Args, ToolResult};

// `[PowerStat]` section options.
#[derive(Deserialize)]
pub(crate) struct PowerstatConfig {
    pub(crate) enabled: bool,
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
    pub(crate) fn run(powerstat_config: &PowerstatConfig) -> ToolResult {
        let powerstat_output = run_command("powerstat", powerstat_config);

        let (body, result) = if powerstat_output.status.success() {
            format_command_output(powerstat_output.stdout)
        } else {
            format_command_output(powerstat_output.stderr)
        };

        ToolResult {
            header: "Powerstat",
            body,
            result,
        }
    }
}
