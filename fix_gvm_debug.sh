#!/bin/bash

# GVM_DEBUG 问题修复脚本
# 在 .bashrc 中添加 GVM_DEBUG 初始化

echo "🔧 开始修复 GVM_DEBUG 问题..."

# 备份 .bashrc
cp ~/.bashrc ~/.bashrc.backup.$(date +%Y%m%d_%H%M%S)
echo "✅ 已备份 .bashrc 到 ~/.bashrc.backup.$(date +%Y%m%d_%H%M%S)"

# 检查是否已经有 GVM_DEBUG 设置
if grep -q "export GVM_DEBUG" ~/.bashrc; then
    echo "⚠️  GVM_DEBUG 已在 .bashrc 中设置"
else
    # 在 GVM 加载之前添加 GVM_DEBUG 初始化
    sed -i '/\[\[ -s "\/home\/lc\/.gvm\/scripts\/gvm" \]\]/i # 初始化 GVM_DEBUG 避免 unbound variable 错误\nexport GVM_DEBUG=${GVM_DEBUG:-0}' ~/.bashrc
    echo "✅ 已在 .bashrc 中添加 GVM_DEBUG 初始化"
fi

echo ""
echo "🔍 验证修复..."
if bash -c 'set -u; source ~/.bashrc' 2>&1 | grep -q "GVM_DEBUG.*unbound"; then
    echo "❌ 修复失败，仍有 GVM_DEBUG unbound 错误"
    exit 1
else
    echo "✅ 修复成功！GVM_DEBUG 错误已解决"
fi

echo ""
echo "📋 修复内容:"
echo "   - 在 .bashrc 中添加了 'export GVM_DEBUG=\${GVM_DEBUG:-0}'"
echo "   - 位置：在 GVM 加载之前"
echo ""
echo "🎉 修复完成！现在可以正常运行命令了"
echo ""
echo "💡 建议测试命令:"
echo "   git status"
echo "   cargo check"
echo "   ./scripts/upload_to_github.sh"
