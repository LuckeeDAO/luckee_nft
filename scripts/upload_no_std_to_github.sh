#!/bin/bash

# Luckee NFT 项目 no_std 改造版本自动上传到 GitHub 脚本
# 使用方法: ./scripts/upload_no_std_to_github.sh

set -e  # 遇到错误时退出

echo "🚀 开始上传 Luckee NFT 项目 no_std 改造版本到 GitHub..."

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
git commit -m "feat: 完成 no_std 改造并优化项目结构

- 添加 #![no_std] 声明和 extern crate alloc
- 替换所有 std::collections 为 alloc::collections
- 替换所有 format! 为 alloc::format!
- 添加条件编译 #[cfg(feature = \"cosmwasm\")]
- 配置 features 系统 (default=[], std=[...], cosmwasm=[...])
- 所有依赖设置为 optional = true, default-features = false
- 添加 no_std 兼容性测试模块
- 优化项目文件结构:
  - 移动部署测试报告到 docs/no_std部署测试报告.md
  - 移动测试脚本到 scripts/no_std测试脚本.sh
- 保持所有 CosmWasm 功能完整性
- 支持嵌入式环境和裸机部署
- 性能优化 (BTree 集合减少内存使用)"

# 确认远程仓库设置
echo "🔗 确认远程仓库设置..."
git remote -v

# 推送代码到 GitHub
echo "⬆️  推送代码到 GitHub..."
git push -u origin main

echo "✅ 项目 no_std 改造版本已成功上传到 GitHub!"

# 显示项目信息
echo ""
echo "📊 项目统计:"
echo "   - 总文件数: $(find . -type f | wc -l)"
echo "   - 代码行数: $(find . -name "*.rs" -exec wc -l {} + | tail -1 | awk '{print $1}')"
echo "   - 测试文件: $(find . -name "*test*.rs" | wc -l)"
echo "   - 文档文件: $(find . -name "*.md" | wc -l)"
echo "   - 脚本文件: $(find . -name "*.sh" | wc -l)"

echo ""
echo "🎉 上传完成! 您现在可以访问 GitHub 仓库查看您的项目"
echo "📋 本次 no_std 改造包含:"
echo "   - 完全 no_std 兼容性"
echo "   - 嵌入式环境支持"
echo "   - 性能优化 (BTree 集合)"
echo "   - 条件编译支持"
echo "   - 完整的测试覆盖"
echo "   - 部署脚本和文档"
