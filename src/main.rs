mod configurator;
mod output;
mod tools;

use std::path::PathBuf;

use clap::Parser;

use minijinja::Environment;

use crate::configurator::Configurator;
use crate::output::{Output, ToolOutput};
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

static TEMPLATES: &[(&str, &str)] = &builtin_templates![
    ("md.report", "report.md"),
    ("md.final_report", "final_report.md")
];

fn validate_configuration_file(configuration_path: &str) -> Result<PathBuf, String> {
    let configuration_path = configuration_path
        .parse::<PathBuf>()
        .map_err(|_| "Invalid configuration path. Insert a path to the `lich.toml` file.")?;

    // Configuration file must be present in the passed directory.
    if configuration_path.is_dir() {
        let configuration_path = configuration_path.join("lich.toml");
        match configuration_path.try_exists() {
            Ok(false) => Err("The configuration path is a directory, but it does not contain the `lich.toml` file.".into()),
            Err(e) => Err(format!("Error checking the configuration path existence: {e}")),
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

fn run_vulnerability_tools(config: &Configurator, environment: &Environment) -> Vec<ToolOutput> {
    let mut vulnerability_tools = Vec::new();
    if config.is_valgrind_enabled() {
        // Check valgrind existence.
        //
        // Block the execution whether the tool is not found.
        Valgrind::check_existence().expect("`valgrind` not found on the system");

        // Run tool.
        let valgrind = Valgrind::run(config);

        // Produce `valgrind` report.
        valgrind.write_report(environment);

        // Add data for final report.
        vulnerability_tools.push(valgrind.final_report_data());
    }

    vulnerability_tools
}

fn run_energy_tools(config: &Configurator, environment: &Environment) -> Vec<ToolOutput> {
    let mut energy_tools = Vec::new();
    if config.is_powerstat_enabled() {
        // Check powerstat existence.
        //
        // Block the execution whether the tool is not found.
        Powerstat::check_existence().expect("`powerstat` not found on the system");

        // Run tool.
        let powerstat = Powerstat::run(config);

        // Produce `powerstat` report.
        powerstat.write_report(environment);

        // Add data for final report.
        energy_tools.push(powerstat.final_report_data());
    }

    if config.is_powertop_enabled() {
        // Check powertop existence.
        //
        // Block the execution whether the tool is not found.
        Powertop::check_existence().expect("`powertop` not found on the system");

        // Run tool.
        let powertop = Powertop::run(config);

        // Produce `powertop` report.
        powertop.write_report(environment);

        // Add data for final report.
        energy_tools.push(powertop.final_report_data());
    }

    energy_tools
}

fn main() {
    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt().init();

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

    // Run vulnerability tools and retrieve their data for the final report.
    let vulnerability_tools = run_vulnerability_tools(&config, &environment);

    // Run energy tools and retrieve their data for the final report.
    let energy_tools = run_energy_tools(&config, &environment);

    // Produce final report.
    Output::produce_final_report(&config, &environment, &vulnerability_tools, &energy_tools);
}
