# rhostname

## Introduction

rhostname is a command-line tool used to display the system's DNS name, hostname, or NIS domain name. It is designed as a more secure and efficient alternative to the traditional hostname command, which has been known to have memory leak issues. By rewriting the functionality in Rust, rhostname eliminates memory leaks, offering a safer and faster solution while maintaining compatibility with all the features of the original hostname command.

## Installation Guide

### Installing from an RPM package

Use `rhostname.spec` to install, depending on the operating system.

### Compiling from Source

1. Clone the project repository:

```bash
git clone https://gitee.com/insorker/rhostname
cd rhostname
```

2. Compile the project using cargo:

```bash
cargo build --release
```

3. Install the binary:

```bash
sudo cp target/release/rhostname /usr/local/bin/
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