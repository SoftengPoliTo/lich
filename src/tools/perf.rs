use std::io::Error;

use minijinja::Environment;

use serde::Deserialize;

use crate::configurator::{always_true, Configurator};
use crate::output::{create_report_path, Output, ToolOutput};

use super::{
    check_tool_existence, run_tool, stderr_output, stdout_stderr_output, sudo_run_tool, Args,
    ToolCommands,
};

const TOOL_NAME: &str = "perf";
const TOOL_HEADER: &str = "Perf";

// `[perf]` section options.
#[derive(Deserialize)]
pub(crate) struct PerfConfig {
    #[serde(default = "always_true")]
    pub(crate) enable: bool,
    #[serde(default = "Vec::new")]
    pub(crate) args: Vec<String>,
}

impl Default for PerfConfig {
    fn default() -> Self {
        Self {
            enable: true,
            args: Vec::new(),
        }
    }
}

impl Args for PerfConfig {
    fn args(&self) -> &[String] {
        &self.args
    }
}

pub(crate) struct Perf<'a> {
    config: &'a Configurator,
    output: String,
    result: &'static str,
    report_path: String,
}

impl<'a> ToolCommands<'a> for Perf<'a> {
    fn check_existence() -> Result<std::process::Output, Error> {
        check_tool_existence(TOOL_NAME)
    }

    fn run(config: &'a Configurator) -> Self {
        let output = if config.root.is_empty() {
            run_tool(
                TOOL_NAME,
                config.perf.args(),
                &config.binary_path,
                config.binary.args(),
            )
        } else {
            sudo_run_tool(
                TOOL_NAME,
                config.perf.args(),
                &config.binary_path,
                config.binary.args(),
                &config.root,
            )
        };

        let (output, result) = if output.status.success() {
            stdout_stderr_output(&output.stdout, &output.stderr)
        } else {
            stderr_output(&output.stderr)
        };

        let report_path = create_report_path(TOOL_NAME, config.format.ext());

        Self {
            config,
            output,
            result,
            report_path,
        }
    }

    fn write_report(&self, environment: &Environment) {
        Output::write_report(
            environment,
            TOOL_HEADER,
            self.result,
            &self.output,
            &self.config.report_path,
            &self.report_path,
        );
    }

    fn final_report_data(self) -> ToolOutput {
        ToolOutput::new(TOOL_HEADER, self.report_path, self.result)
    }
}
