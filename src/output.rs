use std::fs::write;
use std::path::Path;

use minijinja::{context, Environment};

use serde::{Deserialize, Serialize};

use crate::configurator::Configurator;

const FINAL_REPORT_MARKDOWN: &str = "final_report.md";

pub(crate) fn create_report_path(tool_name: &str, extension: &str) -> String {
    format!("{tool_name}.{extension}")
}

#[derive(Serialize)]
pub(crate) struct ToolOutput {
    header: &'static str,
    report_path: String,
    result: &'static str,
}

impl ToolOutput {
    pub(crate) const fn new(
        header: &'static str,
        report_path: String,
        result: &'static str,
    ) -> Self {
        Self {
            header,
            report_path,
            result,
        }
    }
}

#[derive(Default, Deserialize)]
pub(crate) enum ReportFormat {
    #[default]
    #[serde(alias = "markdown")]
    Markdown,
    #[serde(alias = "html")]
    Html,
    #[serde(alias = "all")]
    All,
}

impl ReportFormat {
    pub(crate) const fn ext(&self) -> &'static str {
        if matches!(self, Self::Markdown) {
            "md"
        } else {
            "html"
        }
    }
}

pub(crate) struct Output;

impl Output {
    pub(crate) fn write_report(
        environment: &Environment,
        header: &str,
        result: &str,
        output: &str,
        report_path: &Path,
        report_path_file: &str,
    ) {
        let template = environment.get_template("md.report").unwrap();

        let rendered = template
            .render(context! {
                header => header,
                result => result,
                output => output,
            })
            .unwrap();

        write(report_path.join(report_path_file), rendered).unwrap();
    }

    pub(crate) fn produce_final_report(
        config: &Configurator,
        environment: &Environment,
        vulnerability_tools: &[ToolOutput],
        energy_tools: &[ToolOutput],
    ) {
        // Generate report containing results produced by each tool.
        match config.format {
            // TODO: Move all in another branch when HTML report is implemented.
            ReportFormat::Markdown | ReportFormat::All => {
                Self::generate_markdown_report(
                    &config.report_path,
                    environment,
                    vulnerability_tools,
                    energy_tools,
                );
            }
            ReportFormat::Html => {}
        }
    }

    fn generate_markdown_report(
        final_report_path: &Path,
        environment: &Environment,
        vulnerability_tools: &[ToolOutput],
        energy_tools: &[ToolOutput],
    ) {
        let template = environment.get_template("md.final_report").unwrap();

        let rendered = template
            .render(context! {
                vulnerability_tools => vulnerability_tools,
                energy_tools => energy_tools,
            })
            .unwrap();

        write(final_report_path.join(FINAL_REPORT_MARKDOWN), rendered).unwrap();
    }
}
