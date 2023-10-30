# Rust API helper for creating IQEngine plugin server

## Overview

[IQEngine](https://www.iqengine.org/) is a web-based SDR toolkit for analyzing, processing, and sharing RF recordings.

This crate is an on-going work.
Moreover the [IQEngine plugin server API](https://www.iqengine.org/docs/plugins) is not stabilized yet. Expect breaking change in close future.

The objective of this crate is to leverage the [FutureSDR](https://www.futuresdr.org/) framework to create such plugin server.

![Depiction of Architecture of IQEngine. Most of the process is actually runned in the client browser. Still the plugin server sits between the browser and the IQs Storage. It is HTTP oriented.](https://www.iqengine.org/security_overview.drawio.png)

## Installation

Just use the following command in your project folder:

```sh
cargo add iqengine-plugin
```

## Documentation

The easiest way to start your own IQEngine plugin server is probably to [create your own repository from the template](https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-a-repository-from-a-template) called [iqengine-fm-receiver-plugin](https://github.com/loic-fejoz/iqengine-fm-receiver-plugin/).

Then one has to implement the [trait IQFunction](https://docs.rs/iqengine-plugin/target/latest/iqengine_plugin/server/trait.IQFunction.html) and expose it to proper HTTP API endpoints. Eventually more helpers code will be provided to ease the process.

As a starter, the easiest code to read in order to understand is the [amplifier function source code](https://github.com/loic-fejoz/iqengine-fm-receiver-plugin/blob/main/src/amplifier.rs).

More to come...

## Notes

After having tentatively creating the library with the [OpenAPI Generator](https://openapi-generator.tech) project, the source code has been created manually.
