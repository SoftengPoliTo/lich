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

trait Args {
    fn args(&self) -> &[String];
}

fn run_command<T: Args>(command_name: &str, config: &T, binary_path: &Path) -> Output {
    Command::new(command_name)
        .args(config.args())
        .arg(binary_path)
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
