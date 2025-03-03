use std::fs::write;
use std::path::PathBuf;

use minijinja::{context, Environment};

use serde::Deserialize;

use crate::ToolResult;

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

pub(crate) struct Output {
    report_format: ReportFormat,
    output_path: PathBuf,
}

impl Output {
    pub(crate) fn new(report_format: ReportFormat, output_path: PathBuf) -> Self {
        Self {
            report_format,
            output_path,
        }
    }

    pub(crate) fn run(
        self,
        environment: &Environment,
        vulnerability_tools: &[ToolResult],
        energy_tools: &[ToolResult],
    ) {
        // Generate report containing results produced by each tool.
        match self.report_format {
            // TODO: Move all in another branch when HTML report is implemented.
            ReportFormat::Markdown | ReportFormat::All => {
                self.generate_markdown_report(environment, vulnerability_tools, energy_tools);
            }
            ReportFormat::Html => {}
        }
    }

    fn generate_markdown_report(
        &self,
        environment: &Environment,
        vulnerability_tools: &[ToolResult],
        energy_tools: &[ToolResult],
    ) {
        let template = environment.get_template("md.report").unwrap();

        let rendered = template
            .render(context! {
                vulnerability_tools => vulnerability_tools,
                energy_tools => energy_tools,
            })
            .unwrap();

        write(&self.output_path, rendered).unwrap();
    }
}
