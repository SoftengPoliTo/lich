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

fn format_command_output(output: Vec<u8>) -> (String, &'static str) {
    let str_output = String::from_utf8(output).unwrap();
    let body = format!(
        "```
{str_output}
```"
    );
    let result = "&#x1F600;";
    (body, result)
}
