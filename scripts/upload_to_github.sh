#!/bin/bash

# Luckee NFT 项目自动上传到 GitHub 脚本
# 使用方法: ./scripts/upload_to_github.sh

set -e  # 遇到错误时退出

echo "🚀 开始上传 Luckee NFT 项目到 GitHub..."

# 检查是否在正确的目录
if [ ! -f "Cargo.toml" ]; then
    echo "❌ 错误: 请在项目根目录运行此脚本"
    exit 1
fi

# 检查 Git 状态
echo "📋 检查 Git 状态..."
git status

# 添加所有修改的文件
echo "📝 添加所有修改的文件..."
git add .

# 提交更改
echo "💾 提交更改..."
git commit -m "feat: 升级 cosmwasm-std 到 2.2.2 并修复兼容性问题

- 升级 cosmwasm-std 从 1.5 到 2.2.2
- 升级 cosmwasm-schema 从 1.5 到 2.2.2  
- 升级 cw-storage-plus 从 1.1 到 2.2.2
- 升级 cw721-base 从 0.17 到 0.18
- 升级 cw721 从 0.17 到 0.18
- 升级 cw-utils 从 3.0.0 到 3.2.0
- 升级 cw2 从 1.0 到 2.0
- 升级 cw-multi-test 从 0.16 到 0.18
- 添加升级验证报告和摘要文件
- 确保所有 API 兼容性，无破坏性变更"

# 确认远程仓库设置
echo "🔗 确认远程仓库设置..."
git remote -v

# 推送代码到 GitHub
echo "⬆️  推送代码到 GitHub..."
git push -u origin main

echo "✅ 项目已成功上传到 GitHub!"

# 显示项目信息
echo ""
echo "📊 项目统计:"
echo "   - 总文件数: $(find . -type f | wc -l)"
echo "   - 代码行数: $(find . -name "*.rs" -exec wc -l {} + | tail -1 | awk '{print $1}')"
echo "   - 测试文件: $(find . -name "*test*.rs" | wc -l)"
echo "   - 文档文件: $(find . -name "*.md" | wc -l)"
echo "   - 升级报告: $(find . -name "*upgrade*" | wc -l)"

echo ""
echo "🎉 上传完成! 您现在可以访问 GitHub 仓库查看您的项目"
echo "📋 本次升级包含:"
echo "   - CosmWasm 生态系统升级到 2.2.2"
echo "   - 完整的兼容性验证"
echo "   - 详细的升级报告"
echo "   - 新特性支持 (MessagePack 编码、IBC 费用等)"
