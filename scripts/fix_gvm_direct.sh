#!/bin/bash

# 直接修复GVM脚本中的GVM_DEBUG问题
# 通过修改GVM源码来解决unbound variable错误

set -e

echo "🔧 直接修复GVM脚本中的GVM_DEBUG问题..."

# 设置环境变量
export GVM_DEBUG=0

# 备份原始文件
echo "📋 备份原始GVM脚本..."
if [ -f "/home/lc/.gvm/bin/gvm" ]; then
    cp /home/lc/.gvm/bin/gvm /home/lc/.gvm/bin/gvm.backup.$(date +%Y%m%d_%H%M%S)
    echo "✅ 已备份 /home/lc/.gvm/bin/gvm"
fi

# 修复GVM主脚本
echo "🔧 修复GVM主脚本..."
if [ -f "/home/lc/.gvm/bin/gvm" ]; then
    # 在文件开头添加GVM_DEBUG设置
    sed -i '1i\export GVM_DEBUG=${GVM_DEBUG:-0}' /home/lc/.gvm/bin/gvm
    echo "✅ 已修复 /home/lc/.gvm/bin/gvm"
fi

# 修复其他GVM脚本
echo "🔧 修复其他GVM脚本..."

# 修复env/gvm脚本
if [ -f "/home/lc/.gvm/scripts/env/gvm" ]; then
    cp /home/lc/.gvm/scripts/env/gvm /home/lc/.gvm/scripts/env/gvm.backup.$(date +%Y%m%d_%H%M%S)
    sed -i '1i\export GVM_DEBUG=${GVM_DEBUG:-0}' /home/lc/.gvm/scripts/env/gvm
    echo "✅ 已修复 /home/lc/.gvm/scripts/env/gvm"
fi

# 修复env/cd脚本
if [ -f "/home/lc/.gvm/scripts/env/cd" ]; then
    cp /home/lc/.gvm/scripts/env/cd /home/lc/.gvm/scripts/env/cd.backup.$(date +%Y%m%d_%H%M%S)
    sed -i '1i\export GVM_DEBUG=${GVM_DEBUG:-0}' /home/lc/.gvm/scripts/env/cd
    echo "✅ 已修复 /home/lc/.gvm/scripts/env/cd"
fi

# 修复env/use脚本
if [ -f "/home/lc/.gvm/scripts/env/use" ]; then
    cp /home/lc/.gvm/scripts/env/use /home/lc/.gvm/scripts/env/use.backup.$(date +%Y%m%d_%H%M%S)
    sed -i '1i\export GVM_DEBUG=${GVM_DEBUG:-0}' /home/lc/.gvm/scripts/env/use
    echo "✅ 已修复 /home/lc/.gvm/scripts/env/use"
fi

# 修复env/pkgset-use脚本
if [ -f "/home/lc/.gvm/scripts/env/pkgset-use" ]; then
    cp /home/lc/.gvm/scripts/env/pkgset-use /home/lc/.gvm/scripts/env/pkgset-use.backup.$(date +%Y%m%d_%H%M%S)
    sed -i '1i\export GVM_DEBUG=${GVM_DEBUG:-0}' /home/lc/.gvm/scripts/env/pkgset-use
    echo "✅ 已修复 /home/lc/.gvm/scripts/env/pkgset-use"
fi

echo "🎉 GVM脚本修复完成！"
echo "请重新加载shell配置: source ~/.bashrc"
