mod powerstat;
mod powertop;
mod valgrind;

pub(crate) use powerstat::{Powerstat, PowerstatConfig};
pub(crate) use powertop::{Powertop, PowertopConfig};
pub(crate) use valgrind::{Valgrind, ValgrindConfig};

use std::path::Path;
use std::process::{Command, Output};

use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct ToolResult {
    header: &'static str,
    body: String,
    result: &'static str,
}

pub(crate) trait Args {
    fn args(&self) -> &[String];
}

fn run_command<T: Args, K: Args>(
    command_name: &str,
    config: &T,
    binary_path: &Path,
    binary_config: &K,
) -> Output {
    Command::new(command_name)
        .args(config.args())
        .arg(binary_path)
        .args(binary_config.args())
        .output()
        .unwrap()
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
