mod configuration;
mod output;
mod powerstat;
mod powertop;
mod valgrind;

use std::path::PathBuf;

use clap::Parser;

use minijinja::Environment;

use serde::Serialize;

use crate::configuration::Configuration;
use crate::output::Output;
/*use crate::powerstat::PowerStat;
use crate::powertop::PowerTop;
use crate::valgrind::Valgrind;*/

macro_rules! builtin_templates {
    ($(($name:expr, $template:expr)),+) => {
        [
        $(
            (
                $name,
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/templates/", $template)),
            )
        ),+
        ]
    }
}

static TEMPLATES: &[(&str, &str)] = &builtin_templates![("md.report", "report.md")];

fn validate_binary(binary_path: &str) -> Result<PathBuf, String> {
    let binary_path = binary_path
        .parse::<PathBuf>()
        .map_err(|_| "Invalid binary path. Insert a path to the binary.")?;

    // Binary path must not be a directory.
    if binary_path.is_dir() {
        return Err("The binary path must not be a directory. Insert a path to the binary.".into());
    }

    Ok(binary_path)
}

fn validate_configuration_file(configuration_path: &str) -> Result<PathBuf, String> {
    let configuration_path = configuration_path
        .parse::<PathBuf>()
        .map_err(|_| "Invalid configuration path. Insert a path to the `lich.toml` file.")?;

    // Configuration file path must be a file, not a directory.
    if configuration_path.is_dir() {
        return Err("The configuration path must not be a directory. Insert a path to the configuration file.".into());
    }

    // Configuration file must be called `lich.toml`.
    if !configuration_path.ends_with("lich.toml") {
        return Err(
            "The configuration file must be called `lich.toml`. Rename the file accordingly."
                .into(),
        );
    }

    Ok(configuration_path)
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// The input binary path.
    #[clap(short, long, value_hint = clap::ValueHint::FilePath, value_parser = validate_binary)]
    binary_path: PathBuf,
    /// The configuration file path.
    #[clap(short, long, value_hint = clap::ValueHint::FilePath, value_parser = validate_configuration_file)]
    configuration_path: PathBuf,
}

#[derive(Serialize)]
struct ToolResult {
    header: String,
    body: String,
    result: String,
}

fn main() {
    // Read command line arguments.
    let args = Args::parse();

    // Read configuration file.
    //
    // The configuration file is mandatory.
    let config = Configuration::read(&args.configuration_path);

    let mut environment = Environment::new();
    for (name, src) in TEMPLATES {
        environment
            .add_template(name, src)
            .expect("Internal error, built-in template");
    }

    let mut vulnerability_tools = Vec::new();
    if config.is_valgrind_enabled() {
        //vulnerability_tools.push(Valgrind::result().unwrap());
    }

    let mut energy_tools = Vec::new();
    if config.is_powerstat_enabled() {
        //energy_tools.push(PowerStat::result().unwrap());
    }

    if config.is_powertop_enabled() {
        //energy_tools.push(PowerTop::result().unwrap());
    }

    Output::new(config.format, config.report_path).run(
        &environment,
        &vulnerability_tools,
        &energy_tools,
    );
}
