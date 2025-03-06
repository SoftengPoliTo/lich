mod powerstat;
mod powertop;
mod valgrind;

pub(crate) use powerstat::{Powerstat, PowerstatConfig};
pub(crate) use powertop::{Powertop, PowertopConfig};
pub(crate) use valgrind::{Valgrind, ValgrindConfig};

use std::io::Error;
use std::path::Path;
use std::process::{Command, Output};

use crate::configurator::BinaryConfig;

pub(crate) trait Args {
    fn args(&self) -> &[String];
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
        tool_arguments: &[String],
        binary_input: &str,
        root: &str,
    ) {
        internal_sudo(tool_name, tool_arguments, root);
        tracing::info!("Tool input: {binary_input}");
    }

    pub(super) fn print_input(tool_name: &str, tool_arguments: &[String], binary_input: &str) {
        internal_tool(tool_name, tool_arguments);
        tracing::info!("Tool input: {binary_input}");
    }

    pub(super) fn sudo_print_tool(
        tool_name: &str,
        tool_arguments: &[String],
        binary_path: &Path,
        binary_arguments: &[String],
        root: &str,
    ) {
        internal_sudo(tool_name, tool_arguments, root);
        binary(binary_path, binary_arguments);
    }

    pub(super) fn print_tool(
        tool_name: &str,
        tool_arguments: &[String],
        binary_path: &Path,
        binary_arguments: &[String],
    ) {
        internal_tool(tool_name, tool_arguments);
        binary(binary_path, binary_arguments);
    }

    pub(super) fn print_timeout(
        tool_name: &str,
        timeout: &str,
        tool_arguments: &[String],
        binary_path: &Path,
        binary_arguments: &[String],
    ) {
        tracing::info!("Tool: timeout {timeout} {tool_name}");
        tracing::info!("Tool arguments: {}", tool_arguments.join(" "));
        binary(binary_path, binary_arguments);
    }
}

fn create_tool_output(command_ref: &mut Command) -> Output {
    command_ref.output().unwrap()
}

fn sudo_run_tool_with_input<T: Args>(
    tool_name: &str,
    tool_config: &T,
    binary_input: &str,
    root: &str,
) -> Output {
    #[cfg(feature = "tracing")]
    tracing::sudo_print_input(tool_name, tool_config.args(), binary_input, root);

    create_tool_output(
        Command::new(root)
            .arg(tool_name)
            .args(tool_config.args())
            .arg(binary_input),
    )
}

fn run_tool_with_input<T: Args>(tool_name: &str, tool_config: &T, binary_input: &str) -> Output {
    #[cfg(feature = "tracing")]
    tracing::print_input(tool_name, tool_config.args(), binary_input);

    create_tool_output(
        Command::new(tool_name)
            .args(tool_config.args())
            .arg(binary_input),
    )
}

fn sudo_run_tool<T: Args>(
    tool_name: &str,
    tool_config: &T,
    binary_path: &Path,
    binary_config: &BinaryConfig,
    root: &str,
) -> Output {
    #[cfg(feature = "tracing")]
    tracing::sudo_print_tool(
        tool_name,
        tool_config.args(),
        binary_path,
        binary_config.args(),
        root,
    );

    create_tool_output(
        Command::new(root)
            .arg(tool_name)
            .args(tool_config.args())
            .arg(binary_path)
            .args(binary_config.args()),
    )
}

fn run_tool<T: Args>(
    tool_name: &str,
    tool_config: &T,
    binary_path: &Path,
    binary_config: &BinaryConfig,
) -> Output {
    #[cfg(feature = "tracing")]
    tracing::print_tool(
        tool_name,
        tool_config.args(),
        binary_path,
        binary_config.args(),
    );

    create_tool_output(
        Command::new(tool_name)
            .args(tool_config.args())
            .arg(binary_path)
            .args(binary_config.args()),
    )
}

fn run_tool_with_timeout<T: Args>(
    tool_name: &str,
    tool_config: &T,
    binary_path: &Path,
    binary_config: &BinaryConfig,
    timeout: u16,
) -> Output {
    let timeout = format!("{timeout}s");

    #[cfg(feature = "tracing")]
    tracing::print_timeout(
        tool_name,
        &timeout,
        tool_config.args(),
        binary_path,
        binary_config.args(),
    );

    create_tool_output(
        Command::new("timeout")
            .arg(timeout)
            .arg(tool_name)
            .args(tool_config.args())
            .arg(binary_path)
            .args(binary_config.args()),
    )
}

fn stdout_output(message: Vec<u8>) -> (String, &'static str) {
    let output = String::from_utf8(message).unwrap();
    let result = "[Success &#x1F600;]";
    (output, result)
}

fn stderr_output(message: Vec<u8>) -> (String, &'static str) {
    let output = String::from_utf8(message).unwrap();
    let result = "[Error &#x1F915;]";
    (output, result)
}
