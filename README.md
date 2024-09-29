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
