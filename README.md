# hostname

## 参数处理

hostname 同时只能接收一个参数，多个参数优先处理最后一个，如

```sh
hostname -d -f <=> hostname -f
hostname -f -d <=> hostname -d
```

`hostname -V` 是例外，和别的参数同时出现时优先处理 `-V。

## 设置 hostname

参考 https://askubuntu.com/questions/1343609/sudo-unable-to-resolve-host-hostname-temporary-failure-in-name-resolution 和 https://blog.csdn.net/ichuzhen/article/details/8241847

# sethostname

## 错误类型

rust 的错误类型和 linux 的错误类型不一样，参考 https://doc.rust-lang.org/std/io/enum.ErrorKind.html 和 https://internals.rust-lang.org/t/insufficient-std-io-error/3597

## 如何处理错误

1. 读取 HOST_NAME_MAX 提前判断输入长度是否超出

> POSIX.1 guarantees that "Host names (not includ‐ing the terminating null byte) are limited to HOST_NAME_MAX bytes"

2. 不预处理，直接将参数传入 sethostname，读取返回值，EINVAL 会告诉我们长度是否超出
