mod powerstat;
mod powertop;
mod valgrind;

pub(crate) use powerstat::{Powerstat, PowerstatConfig};
pub(crate) use powertop::{Powertop, PowertopConfig};
pub(crate) use valgrind::{Valgrind, ValgrindConfig};

use std::ffi::OsStr;
use std::io::Error;
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

fn create_tool_output(command_ref: &mut Command) -> Output {
    println!("Complete command: {:?}", command_ref.get_program());
    println!(
        "Args: {:?}",
        command_ref.get_args().collect::<Vec<&OsStr>>()
    );

    command_ref.output().unwrap()
}

fn sudo_run_tool_with_input<T: Args, S: AsRef<OsStr>>(
    tool_name: &str,
    tool_config: &T,
    binary_input: S,
    root: &str,
) -> Output {
    create_tool_output(
        Command::new(root)
            .arg(tool_name)
            .args(tool_config.args())
            .arg(binary_input),
    )
}

fn run_tool_with_input<T: Args, S: AsRef<OsStr>>(
    tool_name: &str,
    tool_config: &T,
    binary_input: S,
) -> Output {
    create_tool_output(
        Command::new(tool_name)
            .args(tool_config.args())
            .arg(binary_input),
    )
}

fn sudo_run_tool<T: Args, S: AsRef<OsStr>>(
    tool_name: &str,
    tool_config: &T,
    binary_path: S,
    binary_config: &BinaryConfig,
    root: &str,
) -> Output {
    create_tool_output(
        Command::new(root)
            .arg(tool_name)
            .args(tool_config.args())
            .arg(binary_path)
            .args(binary_config.args()),
    )
}

fn run_tool<T: Args, S: AsRef<OsStr>>(
    tool_name: &str,
    tool_config: &T,
    binary_path: S,
    binary_config: &BinaryConfig,
) -> Output {
    create_tool_output(
        Command::new(tool_name)
            .args(tool_config.args())
            .arg(binary_path)
            .args(binary_config.args()),
    )
}

fn run_tool_with_timeout<T: Args, S: AsRef<OsStr>>(
    tool_name: &str,
    tool_config: &T,
    binary_path: S,
    binary_config: &BinaryConfig,
    timeout: u16,
) -> Output {
    create_tool_output(
        Command::new("timeout")
            .arg(format!("{timeout}s"))
            .arg(tool_name)
            .args(tool_config.args())
            .arg(binary_path)
            .args(binary_config.args()),
    )
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
