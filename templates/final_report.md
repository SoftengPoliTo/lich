# Final Report

This final report contains a sequence of reports produced executing the tools 
enabled in the `lich.toml` configuration file. Each subsection identifies
a category containing the list of paths to the complete tool reports.

{% if vulnerability_tools | length != 0 -%}

## Vulnerability

{% for tool in vulnerability_tools -%}

{{ tool.header }} {{ tool.result}} -> [{{ tool.report_path }}](./{{ tool.report_path }})

{% endfor %}

{%- endif -%}

{%- if energy_tools | length != 0 -%}

## Energy

{% for tool in energy_tools -%}

{{ tool.header }} {{ tool.result}} -> [{{ tool.report_path }}](./{{ tool.report_path }})

{% endfor %}

{%- endif -%}
