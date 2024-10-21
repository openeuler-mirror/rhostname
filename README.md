# rhostname

## 介绍

rhostname 是一个用来显示系统的 DNS 名称、主机名 (hostname) 或 NIS 域名的命令。与原生的 hostname 命令相比，rhostname 使用 Rust 编写，解决了内存泄漏等问题，提供了更加安全、高效的替代方案。它不仅在功能上完全兼容 hostname，还通过 Rust 语言的内存安全特性提高了整体稳定性和性能。

## 安装教程

### 使用 spec 文件构建 RPM 包安装

使用 `rhostname.spec` 构建 RPM 包。

### 使用 `cargo-generate-rpm` 构建 RPM 包安装

使用 `cargo-generate-rpm` 执行下列操作，最后 RPM 包会在 `target/generate-rpm/rhostname.rpm` 处生成。

```shell
cargo install cargo-generate-rpm
cargo build --release
strip -s target/release/rhostname
cargo generate-rpm
```

### 从源码编译安装

1. 克隆项目

```bash
git clone https://gitee.com/insorker/rhostname
cd rhostname
```

2. 使用 cargo 安装项目

```bash
cargo install --path .
```

## 使用说明

安装完成后，你可以通过命令行使用 rhostname。

### 基本用法

```bash
rhostname
```

这将显示系统的当前主机名，类似于 hostname 命令。

### 查看帮助

```bash
rhostname -h
```

显示所有可用的选项和命令说明。

## 测试

### 正确性测试

进入 `tests` 文件夹，运行 `sudo ./output_test.sh` 命令，得到输出如下

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

这表示所有功能全部运行正确。

### 性能测试

进入 `tests` 文件夹，运行 `sudo ./speed_test.sh` 命令，得到输出文件 `benchmark.md`，内容如下

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

通过分析可得 rust 实现与 c 语言实现性能相当，且 c 语言实现略快。其原因分析认为是 hostname 命令本身属于功能型应用，大部分情形是通过系统调用或者库函数调用的形式输出或修改信息，所以大部分情况无法通过修改代码逻辑从而提高运行效率。由于 rust 语言和 c 语言本身的性能对比，rhostname 的理论效率应等于或略高于 hostname。另外，hostname 命令本身由于缺少内存释放这一步，也在一定程度上减少了运行时间。

实际测试 rhostname 大部分参数的执行时间约为 hostname 命令的 1.3 倍，通过排查发现 rhostname 使用的 clap 库执行时间较长，在只考虑参数读取的情况下执行时间已经长于 hostname 命令本身。

考虑结果是不再做优化，理由如下

- 性能测试脚本本身存在一定误差。
- hostname 不属于对时间要求高的命令，比如读取 hostname 命令，其执行时间已经保持在 0.3 ~ 0.4ms 左右，满足正常需求。
- clap 库减少了 rhostname 的开发复杂度和维护难度，同时优化了 hostname 读取参数的逻辑（hostname 读取参数对顺序有未定义的要求，使用 clap 后使得调用顺序固定）。
- rhostname 在消耗时间长的参数下（如 `-A` 参数）效率优于 hostname，解决了 slow path 的问题。