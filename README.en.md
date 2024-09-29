# rhostname

## Introduction

rhostname is a command-line tool used to display the system's DNS name, hostname, or NIS domain name. It is designed as a more secure and efficient alternative to the traditional hostname command, which has been known to have memory leak issues. By rewriting the functionality in Rust, rhostname eliminates memory leaks, offering a safer and faster solution while maintaining compatibility with all the features of the original hostname command.

## Installation Guide

### Install by Building RPM Package Using Spec File

Use `rhostname.spec` to install.

### Install by Building Using `cargo-generate-rpm`

Execute the following commands using `cargo-generate-rpm`, and the RPM package will be generated at `target/generate-rpm/rhostname.rpm`.

```shell
cargo install cargo-generate-rpm
cargo build --release
strip -s target/release/rhostname
cargo generate-rpm
```

### Install from Source

1. Clone the project repository

```bash
git clone https://gitee.com/insorker/rhostname
cd rhostname
```

2. Install the project using cargo

```bash
cargo build --release
```

## Usage Instructions

Once installed, you can use rhostname from the command line.

### Basic Usage

```bash
rhostname
```

This will display the current system hostname, similar to the hostname command.

### View Help

```bash
rhostname -h
```

This will display all available options and detailed usage information.