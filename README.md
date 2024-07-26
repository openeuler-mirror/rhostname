# hostname

## 参数处理

hostname 同时只能接收一个参数，多个参数优先处理最后一个，如

```sh
hostname -d -f <=> hostname -f
hostname -f -d <=> hostname -d
```

`hostname -V` 是例外，和别的参数同时出现时优先处理 `-V。

这个东西不好处理，因为首先语义上这个是无意义的，如果 -d 和 -f 有先后顺序，那么可以使用 subcommand，但是这里没有先后顺序。第二，clap 也不支持这种的处理，我个人意见是不处理这种情况。

## 设置 hostname 发生错误

前情提要，通过 hostname 设置 hostname 是临时设置，系统重启后会恢复原值。

经过我的测试，hostname 设置成 localhost 或者 /etc/hostname 中的值后，再次更改 hostname 不会报错。否则，比如当前 hostname 是 a，尝试调用 hostname b 则会报错

```sh
sudo: unable to resolve host a: Temporary failure in name resolution
```

错误原因参考 https://askubuntu.com/questions/1343609/sudo-unable-to-resolve-host-hostname-temporary-failure-in-name-resolution 和 https://blog.csdn.net/ichuzhen/article/details/8241847


## hostname -b 和 hostname -F ./empty_file 行为不一致

hostname -b 不接参数会输出 hostname，hostname -F ./empty_file 会报错。

# sethostname

## 错误类型

rust 的错误类型和 linux 的错误类型不一样，参考 https://doc.rust-lang.org/std/io/enum.ErrorKind.html 和 https://internals.rust-lang.org/t/insufficient-std-io-error/3597

解决方法有两种

1. 找到 rust 和 linux 对应的错误类型
2. 通过 [RawOsError](https://doc.rust-lang.org/std/io/type.RawOsError.html#) 处理，即 [raw_os_error](https://doc.rust-lang.org/std/io/struct.Error.html#method.raw_os_error)，这个会返回具体错误类型的数字，libc 中有详细的错误类型，如 https://docs.rs/libc/latest/libc/constant.EINVAL.html

## 如何处理错误

1. 读取 HOST_NAME_MAX 提前判断输入长度是否超出

> POSIX.1 guarantees that "Host names (not includ‐ing the terminating null byte) are limited to HOST_NAME_MAX bytes"

2. 不预处理，直接将参数传入 sethostname，读取返回值，EINVAL 会告诉我们长度是否超出

# getdomainname

## domainname 的最大长度

有的说 64，有的说 255
