# `Lich`

*Defying IoT firmware defects through magical instruments*

## Description

Lich is an IoT firmware analyzer to detect vulnerabilities and extract energy
consumption data.

The main idea behind this software consists of aggregating the results produced
by different tools into a single final report. To create this report, all
selected tools must be executed one after the other.
A tool can be disabled from being executed through a configuration file option.
This offers more flexibility in case of unmaintained tools.

It does not matter the programming language adopted for firmware development,
only the final binary instance, and its variants, are taken into consideration
during the analysis.

Lich differs from other tools because developers do not need to install
none of the tools on their own computers since all tests are performed within
a single and independent environment created with Docker.

The final results are presented as a single markdown where at each test is
assigned a different file section.

Lich has been written in Rust because it allows to manage in a safe way a tool
invocation. Indeed, the Rust `Command` API defines a process builder which
provides a fine-grained control over how a new process should be spawned.
Furthermore, a tool written in Rust reduces the possibility of introducing new
vulnerabilities within the tool itself.

## Usability

Lich can also be used for Continuous Integration purposes. It can be run either
at each commit or before an IoT firmware release to verify the presence of
some internal vulnerabilities, but even for estimating firmware energy
consumptions.
Final results can be provides as Continuous Integration artifacts or shown as
a pull request comment in the form of a link pointing to a temporary
`html` file.

This tool might also be adopted within a certification process to evaluate
whether some properties are satisfied or not. When a property is not correct,
the relative test **should** fail.

Energy costs nowadays. Having the possibility to estimate how much an IoT
firmware consumes during its execution helps to reduce the monthly billing,
especially if the firmware must be used day and night without interruptions.

## Limitations

- Lich **must** have the same IoT firmware hardware architecture. Since most of
the tests are performed at runtime, it is needed to run the input binary.
An emulator architecture might not always be the right solution due to the
flaws which usually affect these kinds of software.
- Some of the tools used by Lich might support only the most famous hardware
architectures. Therefore, some peculiar and independent platforms will be
excluded from the analysis.
