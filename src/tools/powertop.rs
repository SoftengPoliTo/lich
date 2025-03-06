use std::io::Error;
use std::path::Path;

use minijinja::Environment;

use serde::Deserialize;

use crate::configurator::{always_true, Configurator};
use crate::output::{create_report_path, Output, ToolOutput};

use super::{
    check_tool_existence, run_tool_with_input, stderr_output, stdout_output,
    sudo_run_tool_with_input, Args,
};

const TOOL_NAME: &str = "powertop";
const TOOL_HEADER: &str = "Powertop";

// `[powertop]` section options.
#[derive(Deserialize)]
pub(crate) struct PowertopConfig {
    #[serde(default = "always_true")]
    pub(crate) enable: bool,
    #[serde(default = "Vec::new")]
    pub(crate) args: Vec<String>,
}

impl Default for PowertopConfig {
    fn default() -> Self {
        Self {
            enable: true,
            args: Vec::new(),
        }
    }
}

impl Args for PowertopConfig {
    fn args(&self) -> &[String] {
        &self.args
    }
}

pub(crate) struct Powertop<'a> {
    config: &'a Configurator,
    output: String,
    result: &'static str,
    report_path: String,
}

impl<'a> Powertop<'a> {
    pub(crate) fn check_existence() -> Result<std::process::Output, Error> {
        check_tool_existence(TOOL_NAME)
    }

    pub(crate) fn run(config: &'a Configurator) -> Self {
        let binary_input = Self::create_binary_input(&config.binary_path, config.binary.args());

        let output = if config.root.is_empty() {
            run_tool_with_input(TOOL_NAME, &config.powertop, binary_input)
        } else {
            sudo_run_tool_with_input(TOOL_NAME, &config.powertop, binary_input, &config.root)
        };

        let (output, result) = if output.status.success() {
            stdout_output(output.stdout)
        } else {
            stderr_output(output.stderr)
        };

        let report_path = create_report_path(TOOL_NAME, config.format.ext());

        Self {
            config,
            output,
            result,
            report_path,
        }
    }

    pub(crate) fn write_report(&self, environment: &Environment) {
        Output::write_report(
            environment,
            TOOL_HEADER,
            self.result,
            &self.output,
            &self.config.report_path,
            &self.report_path,
        );
    }

    pub(crate) fn final_report_data(self) -> ToolOutput {
        ToolOutput::new(TOOL_HEADER, self.report_path, self.result)
    }

    fn create_binary_input(binary_path: &Path, binary_arguments: &[String]) -> String {
        let arguments = binary_arguments.join(" ");
        format!("{} {arguments}", binary_path.to_str().unwrap())
    }
}
