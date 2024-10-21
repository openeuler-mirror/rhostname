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

## Testing

### Correctness Test

Enter the `tests` folder and run the command `sudo ./output_test.sh`. The output will be as follows:

```
test get
Yes
Yes
Yes
Yes
Yes
Yes
Yes
Yes
Yes
test set
Yes
Yes
Yes
```

This indicates that all functions are working correctly.

### Performance Test

Enter the `tests` folder and run the command `sudo ./speed_test.sh`. The output will be saved in the file `benchmark.md`, with content as follows.

#### Hostname vs rhostname Performance Benchmark

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `hostname ` | 0.8 ± 0.9 | 0.5 | 26.8 | 1.00 |
| `../target/release/rhostname ` | 0.9 ± 0.2 | 0.7 | 7.3 | 1.06 ± 1.18 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `hostname -a` | 0.7 ± 0.1 | 0.6 | 5.0 | 1.00 |
| `../target/release/rhostname -a` | 0.9 ± 0.3 | 0.7 | 10.3 | 1.30 ± 0.47 |

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `hostname -A` | 4.481 ± 3.994 | 1.023 | 10.011 | 2.74 ± 2.94 |
| `../target/release/rhostname -A` | 1.638 ± 0.989 | 1.022 | 3.072 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `hostname -d` | 0.7 ± 0.1 | 0.6 | 1.4 | 1.00 |
| `../target/release/rhostname -d` | 0.8 ± 0.1 | 0.7 | 5.0 | 1.25 ± 0.18 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `hostname -f` | 0.7 ± 0.0 | 0.6 | 1.4 | 1.00 |
| `../target/release/rhostname -f` | 0.8 ± 0.1 | 0.7 | 1.6 | 1.28 ± 0.12 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `hostname -i` | 0.7 ± 0.1 | 0.6 | 2.7 | 1.00 |
| `../target/release/rhostname -i` | 0.9 ± 0.1 | 0.7 | 5.1 | 1.27 ± 0.22 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `hostname -I` | 0.6 ± 0.1 | 0.5 | 1.4 | 1.00 |
| `../target/release/rhostname -I` | 0.8 ± 0.1 | 0.7 | 2.1 | 1.28 ± 0.16 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `hostname -s` | 0.6 ± 0.1 | 0.5 | 1.1 | 1.00 |
| `../target/release/rhostname -s` | 0.8 ± 0.1 | 0.6 | 1.5 | 1.32 ± 0.15 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `hostname -y` | 0.6 ± 0.2 | 0.5 | 5.8 | 1.00 |
| `../target/release/rhostname -y` | 0.8 ± 0.1 | 0.7 | 1.8 | 1.28 ± 0.40 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `hostname  hostname` | 0.6 ± 0.1 | 0.5 | 1.6 | 1.00 |
| `../target/release/rhostname  hostname` | 0.8 ± 0.1 | 0.6 | 2.5 | 1.36 ± 0.22 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `hostname -b hostname` | 0.6 ± 0.1 | 0.5 | 4.0 | 1.00 |
| `../target/release/rhostname -b hostname` | 0.8 ± 0.1 | 0.7 | 3.3 | 1.40 ± 0.34 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `hostname -F hostname` | 0.6 ± 0.1 | 0.5 | 1.4 | 1.00 |
| `../target/release/rhostname -F hostname` | 0.8 ± 0.1 | 0.7 | 1.7 | 1.27 ± 0.18 |

Through analysis, it can be concluded that the performance of the Rust implementation is comparable to the C implementation, with the C implementation being slightly faster. The reason for this is that the `hostname` command itself is a functional application, and in most cases, it outputs or modifies information through system calls or library function calls. Therefore, in most situations, it is difficult to improve runtime efficiency by changing the code logic. Given the performance comparison between Rust and C languages, the theoretical efficiency of `rhostname` should be equal to or slightly higher than `hostname`. Additionally, since the `hostname` command lacks memory deallocation, it also reduces the runtime to a certain extent.

Actual tests show that the execution time of most parameters in `rhostname` is about 1.3 times that of the `hostname` command. After investigation, it was found that the clap library used by `rhostname` has a longer execution time, and when only considering parameter reading, its execution time is already longer than that of the `hostname` command.

The decision was made not to optimize further, for the following reasons:

There is some margin of error in the performance test script itself.
The `hostname` command is not time-sensitive. For example, reading the `hostname` command takes about 0.3 to 0.4 ms, which meets normal requirements.
The clap library reduces the complexity of developing and maintaining `rhostname`, while optimizing the logic for reading parameters in `hostname` (the `hostname` command has an undefined requirement for parameter order, and using clap ensures a fixed call order).
`rhostname` performs better than `hostname` for time-consuming parameters (e.g., the `-A` parameter), solving the slow path issue.