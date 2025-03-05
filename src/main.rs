mod configurator;
mod output;
mod tools;

use std::path::PathBuf;

use clap::Parser;

use minijinja::Environment;

use crate::configurator::Configurator;
use crate::output::Output;
use crate::tools::{Powerstat, Powertop, Valgrind};

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

fn validate_configuration_file(configuration_path: &str) -> Result<PathBuf, String> {
    let configuration_path = configuration_path
        .parse::<PathBuf>()
        .map_err(|_| "Invalid configuration path. Insert a path to the `lich.toml` file.")?;

    // Configuration file must be present in the passed directory.
    if configuration_path.is_dir() {
        let configuration_path = configuration_path.join("lich.toml");
        match configuration_path.try_exists() {
            Ok(false) => Err("The configuration path is a directory, but it does not contain any `lich.toml` file.".into()),
            Err(e) => Err(format!("Error checking the configuration path: {e}")),
            _  => Ok(configuration_path)
        }
    } else if configuration_path.is_file() && !configuration_path.ends_with("lich.toml") {
        // Configuration file must be called `lich.toml`.
        Err(
            "The configuration file must be called `lich.toml`. Rename the file accordingly."
                .into(),
        )
    } else {
        Ok(configuration_path)
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// The configuration file path.
    #[clap(short, long = "configuration", value_hint = clap::ValueHint::FilePath, value_parser = validate_configuration_file)]
    configuration_path: PathBuf,
}

fn main() {
    // Read command line arguments.
    let args = Args::parse();

    // Read configuration file.
    //
    // The configuration file is mandatory.
    let config = Configurator::read(&args.configuration_path);

    let mut environment = Environment::new();
    for (name, src) in TEMPLATES {
        environment
            .add_template(name, src)
            .expect("Internal error, built-in template");
    }

    let mut vulnerability_tools = Vec::new();
    if config.is_valgrind_enabled() {
        vulnerability_tools.push(Valgrind::run(
            &config.valgrind,
            &config.binary_path,
            &config.binary,
        ));
    }

    let mut energy_tools = Vec::new();
    if config.is_powerstat_enabled() {
        energy_tools.push(Powerstat::run(
            &config.powerstat,
            &config.binary_path,
            &config.binary,
        ));
    }

    if config.is_powertop_enabled() {
        energy_tools.push(Powertop::run(
            &config.powertop,
            &config.binary_path,
            &config.binary,
        ));
    }

    Output::new(config.format, config.report_path).generate(
        &environment,
        &vulnerability_tools,
        &energy_tools,
    );
}
