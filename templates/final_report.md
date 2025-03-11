# Final Report

This final report contains a sequence of reports produced executing the tools
enabled in `lich.toml` configuration file. Each subsection identifies a
different kind of software assessment and it consists of a list of paths
pointing to complete tools reports.

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
