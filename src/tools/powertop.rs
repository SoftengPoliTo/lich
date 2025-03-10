use std::fs::remove_file;
use std::io::Error;
use std::path::{Path, PathBuf};

use minijinja::Environment;

use serde::Deserialize;

use crate::configurator::{always_true, Configurator};
use crate::output::{create_report_path, Output, ToolOutput};

use super::{
    check_tool_existence, read_file_to_string, run_tool_with_input, stderr_output, stdout_result,
    sudo_run_tool_with_input, Args,
};

const TOOL_NAME: &str = "powertop";
const TOOL_HEADER: &str = "Powertop";
const TOOL_CSV_OUTPUT: &str = "powertop.csv";

fn default_csv_output() -> PathBuf {
    PathBuf::from(TOOL_CSV_OUTPUT)
}

// `[powertop]` section options.
#[derive(Deserialize)]
pub(crate) struct PowertopConfig {
    #[serde(default = "always_true")]
    pub(crate) enable: bool,
    #[serde(default = "Vec::new")]
    pub(crate) args: Vec<String>,
    #[serde(rename = "csv-output")]
    #[serde(default = "default_csv_output")]
    pub(crate) csv_output: PathBuf,
}

impl PowertopConfig {
    pub(crate) fn check_csv_output_path(&self) -> bool {
        self.csv_output
            .extension()
            .is_some_and(|value| value == "csv")
    }

    pub(crate) fn add_csv_output_to_args(&mut self) {
        self.args.push(format!(
            "--csv={}",
            self.csv_output.to_str().unwrap_or(TOOL_CSV_OUTPUT)
        ));
    }
}

impl Default for PowertopConfig {
    fn default() -> Self {
        Self {
            enable: true,
            args: Vec::new(),
            csv_output: default_csv_output(),
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
            run_tool_with_input(TOOL_NAME, config.powertop.args(), &binary_input)
        } else {
            sudo_run_tool_with_input(
                TOOL_NAME,
                config.powertop.args(),
                &binary_input,
                &config.root,
            )
        };

        let (output, result) = if output.status.success() {
            let output = read_file_to_string(&config.powertop.csv_output);

            // Remove output file
            remove_file(&config.powertop.csv_output).unwrap();

            (output, stdout_result())
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
