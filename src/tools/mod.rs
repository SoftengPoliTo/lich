mod powerstat;
mod powertop;
mod valgrind;

pub(crate) use powerstat::{Powerstat, PowerstatConfig};
pub(crate) use powertop::{Powertop, PowertopConfig};
pub(crate) use valgrind::{Valgrind, ValgrindConfig};

use std::io::Error;
use std::path::Path;
use std::process::{Command, Output};

use serde::Serialize;

use crate::configurator::BinaryConfig;

#[derive(Serialize)]
pub(crate) struct ToolResult {
    header: &'static str,
    body: String,
    result: &'static str,
}

pub(crate) trait Args {
    fn args(&self) -> &[String];
}

fn check_tool_existence(tool_name: &str) -> Result<Output, Error> {
    Command::new(tool_name).arg("-v").output()
}

fn run_tool_with_timeout(
    tool_name: &str,
    tool_arguments: &[String],
    binary_path: &Path,
    binary_arguments: &[String],
    timeout: u16,
) -> Output {
    Command::new("timeout")
        .arg(format!("{timeout}s"))
        .arg(tool_name)
        .args(tool_arguments)
        .arg(binary_path)
        .args(binary_arguments)
        .output()
        .unwrap()
}

fn run_tool_only(
    tool_name: &str,
    tool_arguments: &[String],
    binary_path: &Path,
    binary_arguments: &[String],
) -> Output {
    Command::new(tool_name)
        .args(tool_arguments)
        .arg(binary_path)
        .args(binary_arguments)
        .output()
        .unwrap()
}

fn run_tool<T: Args>(
    tool_name: &str,
    tool_config: &T,
    binary_path: &Path,
    binary_config: &BinaryConfig,
) -> Output {
    if binary_config.timeout > 0 {
        run_tool_with_timeout(
            tool_name,
            tool_config.args(),
            binary_path,
            binary_config.args(),
            binary_config.timeout,
        )
    } else {
        run_tool_only(
            tool_name,
            tool_config.args(),
            binary_path,
            binary_config.args(),
        )
    }
}

fn create_body(message: Vec<u8>) -> String {
    let str_output = String::from_utf8(message).unwrap();
    format!(
        "```
{str_output}
```"
    )
}

fn stdout_output(message: Vec<u8>) -> (String, &'static str) {
    let body = create_body(message);
    let result = "[Success &#x1F600;]";
    (body, result)
}

fn stderr_output(message: Vec<u8>) -> (String, &'static str) {
    let body = create_body(message);
    let result = "[Error &#x1F915;]";
    (body, result)
}
