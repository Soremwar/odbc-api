# ODBC-API

[![Docs](https://docs.rs/odbc-api/badge.svg)](https://docs.rs/odbc-api/)
[![Licence](https://img.shields.io/crates/l/odbc-api)](https://github.com/pacman82/odbc-api/blob/master/License)
[![Crates.io](https://img.shields.io/crates/v/odbc-api)](https://crates.io/crates/odbc-api)
[![Coverage Status](https://coveralls.io/repos/github/pacman82/odbc-api/badge.svg?branch=master)](https://coveralls.io/github/pacman82/odbc-api?branch=master)

Rust ODBC bindings. ODBC (Open Database Connectivity) is an open standard to connect to a variaty of data sources. Most data sources offer ODBC drivers. This crate is currently tested against:

* Microsoft SQL Server
* MariaDB
* SQLite

Current ODBC Version is `3.80`.

This crate is build on top of the `odbc-sys` ffi bindings, which provide definitions of the ODBC C Interface, but do not build any kind of abstraction on top of it.

## Usage

Check the [guide](https://docs.rs/odbc-api/latest/odbc_api/guide/index.html) for code examples and a tour of the features.

## Installation

To build this library you need to link against the `odbc` library of your systems ODBC driver manager. It should be automatically detected by the build. On Windows systems it is preinstalled. On Linux and OS-X [unix-odbc](http://www.unixodbc.org/) must be installed. To create a Connections to a data source, its ODBC driver must also be installed.

### Windows

Nothing to do. ODBC driver manager is preinstalled.

### Ubuntu

```shell
sudo apt-get install unixodbc-dev
```

### OS-X (intel)

You can use homebrew to install UnixODBC

```shell
brew install unixodbc
```

### OS-X (ARM / MAC M1)

`cargo build` is not going to pick up `libodbc.so` installed via homebrew due to the fact that homebrew on ARM Mac installs into `/opt/homebrew/Cellar` as opposed to `/usr/local/opt/`.

You find documentation on what directories are searched during build here: <https://doc.rust-lang.org/cargo/reference/environment-variables.html#dynamic-library-paths>.

You can also install unixODBC from source:

1. copy the unixODBC-2.3.9.tar.gz file somewhere you can create files and directories
2. gunzip unixODBC*.tar.gz
3. tar xvf unixODBC*.tar
4. `./configure`
5. `make`
6. `make install`

## Features

* [x] Connect using either Data Source names (DSN)
* [x] Connect using connection strings
* [x] Connect using prompts (windows)
* [x] Support for logging ODBC diagnostics and warnings (via `log` crate).
* [x] Support for columnar bulk inserts.
* [x] Support for columnar bulk queries.
* [ ] Support for rowise bulk inserts.
* [ ] Support for rowise bulk queries.
* [x] Support for Output parameters of stored procedures.
* [x] Support prepared and 'one shot' queries.
* [x] Transactions
* [x] Pass parameters to queries
* [ ] Support for async
* [x] Support for Multithreading
* [x] Support for inserting large binary / text data in stream
* [x] Support for fetching arbitrary large text / binary data in stream
* [x] Support for connection pooling
* [x] List tables of data sources
