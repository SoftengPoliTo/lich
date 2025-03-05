# Final Report

This report presents the results of all those tools which have not been
disabled in the `lich.toml` configuration file. Each result has been assigned
to a specific subsection according to the tool category.

{% if vulnerability_tools | length != 0 -%}

## Vulnerability

{% for tool in vulnerability_tools -%}

### {{ tool.header }} {{ tool.result}}

{{ tool.body }}

{%- endfor -%}

{%- endif -%}

{% if energy_tools | length != 0 -%}

## Energy

{% for tool in energy_tools -%}

## {{ tool.header }} {{ tool.result}}

{{ tool.body }}

{%- endfor -%}

{%- endif -%}
