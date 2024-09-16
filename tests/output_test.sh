#!/bin/bash

commands=(
  "hostname"
  "../target/release/rhostname"
)

# 其中 -A 参数执行时间较长
get_params=(
  ""
  "-a"
  "-A"
  "-d"
  "-f"
  "-i"
  "-I"
  "-s"
  "-y"
)

set_params=(
  ""
  "-b"
  "-F"
)

echo "test get"
for param in "${get_params[@]}"; do
  cmd1=$(${commands[0]} $param)
  cmd2=$(${commands[1]} $param)

  test "$cmd1" = "$cmd2" && echo Yes || echo No
done

if [ "$EUID" -ne 0 ]; then
  echo "请使用 sudo 权限运行剩余脚本"
  exit 1
else
  echo "test set"
  for param in "${set_params[@]}"; do
    hostname "localhost"

    ${commands[0]} $param "hostname"
    ${commands[1]} $param "hostname"
    cmd1=$(${commands[0]})
    cmd2=$(${commands[1]})

    test "$cmd1" = "$cmd2" && echo Yes || echo No
  done
fi