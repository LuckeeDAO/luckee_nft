#!/bin/bash

# GVM 手动升级脚本
# 绕过环境变量问题，直接升级GVM

set -e

echo "🚀 开始手动升级GVM..."

# 设置环境变量
export GVM_DEBUG=0
export GVM_ROOT=/home/lc/.gvm

# 备份当前GVM
echo "📋 备份当前GVM..."
if [ -d "/home/lc/.gvm" ]; then
    cp -r /home/lc/.gvm /home/lc/.gvm.backup.$(date +%Y%m%d_%H%M%S)
    echo "✅ GVM配置已备份"
fi

# 检查当前版本
echo "📊 检查当前版本..."
if [ -f "/home/lc/.gvm/VERSION" ]; then
    CURRENT_VERSION=$(cat /home/lc/.gvm/VERSION)
    echo "当前版本: $CURRENT_VERSION"
fi

# 下载最新GVM
echo "⬇️ 下载最新GVM..."
cd /tmp
curl -s -S -L https://raw.githubusercontent.com/moovweb/gvm/master/binscripts/gvm-installer > gvm-installer.sh

if [ $? -eq 0 ]; then
    echo "✅ GVM安装脚本下载成功"
else
    echo "❌ GVM安装脚本下载失败"
    exit 1
fi

# 执行安装
echo "🔧 执行GVM安装..."
bash gvm-installer.sh

if [ $? -eq 0 ]; then
    echo "✅ GVM安装成功"
else
    echo "❌ GVM安装失败"
    exit 1
fi

# 检查新版本
echo "📊 检查新版本..."
if [ -f "/home/lc/.gvm/VERSION" ]; then
    NEW_VERSION=$(cat /home/lc/.gvm/VERSION)
    echo "新版本: $NEW_VERSION"
    
    if [ "$NEW_VERSION" != "$CURRENT_VERSION" ]; then
        echo "✅ GVM升级成功: $CURRENT_VERSION -> $NEW_VERSION"
    else
        echo "⚠️ GVM版本未变化"
    fi
fi

# 清理临时文件
rm -f /tmp/gvm-installer.sh

echo "🎉 GVM升级完成！"
echo "请重新加载shell配置: source ~/.bashrc"
