#!/bin/bash
# 需要命令行工具 hyperfine (https://github.com/sharkdp/hyperfine)

# 定义输出 Markdown 文件
output_file="benchmark.md"

# 写入 Markdown 文件的头部信息
echo "# Hostname vs rhostname Performance Benchmark" > $output_file
echo "" >> $output_file

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

# 遍历每个命令及参数进行性能测试
for param in "${get_params[@]}"; do
  cmd1="${commands[0]} $param"
  cmd2="${commands[1]} $param"

  hyperfine  -N -w 10 "$cmd1" "$cmd2" --export-markdown temp.md

  cat temp.md >> $output_file
  echo "" >> $output_file
done

if [ "$EUID" -ne 0 ]; then
  echo "请使用 sudo 权限运行剩余脚本"
else
  for param in "${set_params[@]}"; do
    hostname "localhost"

    cmd1="${commands[0]} $param hostname"
    cmd2="${commands[1]} $param hostname"

    hyperfine  -N -w 10 "$cmd1" "$cmd2" --export-markdown temp.md

    cat temp.md >> $output_file
    echo "" >> $output_file
  done
fi

rm temp.md

echo "性能测试完成，结果已输出到 $output_file"
