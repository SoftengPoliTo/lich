mod perf;
mod powerstat;
mod powertop;
mod valgrind;

pub(crate) use perf::{Perf, PerfConfig};
pub(crate) use powerstat::{Powerstat, PowerstatConfig};
pub(crate) use powertop::{Powertop, PowertopConfig};
pub(crate) use valgrind::{Valgrind, ValgrindConfig};

use std::fs::read_to_string;
use std::io::Error;
use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::str::from_utf8;

use minijinja::Environment;

use crate::configurator::Configurator;
use crate::output::ToolOutput;

#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";
#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";

pub(crate) trait Args {
    fn args(&self) -> &[String];
}

pub(crate) trait ToolCommands<'a> {
    fn check_existence() -> Result<Output, Error>;
    fn run(config: &'a Configurator) -> Self;
    fn write_report(&self, environment: &Environment);
    fn final_report_data(self) -> ToolOutput;
}

fn read_file_to_string(path: &Path) -> String {
    read_to_string(path).unwrap()
}

fn create_binary_input(binary_path: &Path, binary_arguments: &[String]) -> String {
    let arguments = binary_arguments.join(" ");
    format!("{} {arguments}", binary_path.to_str().unwrap())
}

fn check_tool_existence(tool_name: &str) -> Result<Output, Error> {
    Command::new(tool_name).arg("-v").output()
}

#[cfg(feature = "tracing")]
mod tracing {
    use super::Path;

    fn internal_sudo(tool_name: &str, tool_args: &[String], root: &str) {
        tracing::info!("Tool: {root} {tool_name}");
        tracing::info!("Tool arguments: {}", tool_args.join(" "));
    }

    fn internal_tool(tool_name: &str, tool_args: &[String]) {
        tracing::info!("Tool: {tool_name}");
        tracing::info!("Tool arguments: { }", tool_args.join(" "));
    }

    fn binary(binary_path: &Path, binary_args: &[String]) {
        tracing::info!("Tool input: {:?} {}", binary_path, binary_args.join(" "));
    }

    pub(super) fn sudo_print_input(
        tool_name: &str,
        tool_args: &[String],
        binary_input: &str,
        root: &str,
    ) {
        internal_sudo(tool_name, tool_args, root);
        tracing::info!("Tool input: {binary_input}");
    }

    pub(super) fn print_input(tool_name: &str, tool_args: &[String], binary_input: &str) {
        internal_tool(tool_name, tool_args);
        tracing::info!("Tool input: {binary_input}");
    }

    pub(super) fn sudo_print_tool(
        tool_name: &str,
        tool_args: &[String],
        binary_path: &Path,
        binary_args: &[String],
        root: &str,
    ) {
        internal_sudo(tool_name, tool_args, root);
        binary(binary_path, binary_args);
    }

    pub(super) fn print_tool(
        tool_name: &str,
        tool_args: &[String],
        binary_path: &Path,
        binary_args: &[String],
    ) {
        internal_tool(tool_name, tool_args);
        binary(binary_path, binary_args);
    }

    pub(super) fn print_timeout(
        tool_name: &str,
        timeout: &str,
        tool_args: &[String],
        binary_path: &Path,
        binary_args: &[String],
    ) {
        tracing::info!("Tool: timeout {timeout} {tool_name}");
        tracing::info!("Tool arguments: {}", tool_args.join(" "));
        binary(binary_path, binary_args);
    }
}

fn create_tool_output(command_ref: &mut Command) -> Output {
    let output = command_ref
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    println!();
    output
}

fn sudo_run_tool_with_input(
    tool_name: &str,
    tool_args: &[String],
    binary_input: &str,
    root: &str,
) -> Output {
    #[cfg(feature = "tracing")]
    tracing::sudo_print_input(tool_name, tool_args, binary_input, root);

    create_tool_output(
        Command::new(root)
            .arg(tool_name)
            .args(tool_args)
            .arg(binary_input),
    )
}

fn run_tool_with_input(tool_name: &str, tool_args: &[String], binary_input: &str) -> Output {
    #[cfg(feature = "tracing")]
    tracing::print_input(tool_name, tool_args, binary_input);

    create_tool_output(Command::new(tool_name).args(tool_args).arg(binary_input))
}

fn sudo_run_tool(
    tool_name: &str,
    tool_args: &[String],
    binary_path: &Path,
    binary_args: &[String],
    root: &str,
) -> Output {
    #[cfg(feature = "tracing")]
    tracing::sudo_print_tool(tool_name, tool_args, binary_path, binary_args, root);

    create_tool_output(
        Command::new(root)
            .arg(tool_name)
            .args(tool_args)
            .arg(binary_path)
            .args(binary_args),
    )
}

fn run_tool(
    tool_name: &str,
    tool_args: &[String],
    binary_path: &Path,
    binary_args: &[String],
) -> Output {
    #[cfg(feature = "tracing")]
    tracing::print_tool(tool_name, tool_args, binary_path, binary_args);

    create_tool_output(
        Command::new(tool_name)
            .args(tool_args)
            .arg(binary_path)
            .args(binary_args),
    )
}

fn run_tool_with_timeout(
    tool_name: &str,
    tool_args: &[String],
    binary_path: &Path,
    binary_args: &[String],
    timeout: u16,
) -> Output {
    let timeout = format!("{timeout}s");

    #[cfg(feature = "tracing")]
    tracing::print_timeout(tool_name, &timeout, tool_args, binary_path, binary_args);

    create_tool_output(
        Command::new("timeout")
            .arg(timeout)
            .arg(tool_name)
            .args(tool_args)
            .arg(binary_path)
            .args(binary_args),
    )
}

fn remove_trailing_whitespaces(message: &str) -> String {
    message
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join(LINE_ENDING)
}

fn format_output(message: &[u8]) -> String {
    remove_trailing_whitespaces(from_utf8(message).unwrap().trim())
}

fn stdout_result() -> &'static str {
    "[Success 😁]"
}

fn stdout_output(message: &[u8]) -> (String, &'static str) {
    let output = format_output(message);
    let result = stdout_result();
    (output, result)
}

fn stderr_output(message: &[u8]) -> (String, &'static str) {
    let output = format_output(message);
    let result = "[Error 🤕]";
    (output, result)
}

fn stdout_stderr_output(stdout_message: &[u8], stderr_message: &[u8]) -> (String, &'static str) {
    let stdout_output = format_output(stdout_message);
    let stderr_output = format_output(stderr_message);

    // Add a white line independently of the operating system in use.
    let output = format!("{stdout_output}{LINE_ENDING}{LINE_ENDING}{stderr_output}");

    (output, stdout_result())
}
