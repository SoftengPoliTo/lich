mod powerstat;
mod powertop;
mod valgrind;

pub(crate) use powerstat::{Powerstat, PowerstatConfig};
pub(crate) use powertop::{Powertop, PowertopConfig};
pub(crate) use valgrind::{Valgrind, ValgrindConfig};

use std::ffi::OsStr;
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

fn run_tool<T: Args>(
    tool_name: &str,
    tool_config: &T,
    binary_path: &Path,
    binary_config: &BinaryConfig,
) -> Output {
    let mut command = Command::new(if binary_config.timeout > 0 {
        "timeout"
    } else {
        tool_name
    });

    let command_ref = if binary_config.timeout > 0 {
        command
            .arg(format!("{}s", binary_config.timeout))
            .arg(tool_name)
    } else {
        &mut command
    };

    let command_ref = command_ref
        .args(tool_config.args())
        .arg(binary_path)
        .args(binary_config.args());

    println!("Complete command: {:?}", command_ref.get_program());
    println!(
        "Args: {:?}",
        command_ref.get_args().collect::<Vec<&OsStr>>()
    );

    command_ref.output().unwrap()
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
