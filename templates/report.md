# Final Report

This report presents the results of all those tools which have not been
voluntarily disabled in the `lich.toml` configuration file. Each result is
assigned to a specific subsection depending on the category associated with its
tool.

{% if vulnerability.tools | length != 0 %}

## Vulnerability

{% for tool in vulnerability_tools %}

### {{ tool.header }}

{{ tool.body }}

**Final result**: {{ tool.result}}

{% endfor %}

{% endif %}

{% if energy.tools | length != 0 %}

## Energy

{% for tool in energy_tools %}

## {{ tool.header }}

{{ tool.body }}

**Final result**: {{ tool.result}}

{% endfor %}

{% endif %}
