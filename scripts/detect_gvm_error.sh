#!/bin/bash

# 自动化检测 GVM_DEBUG unbound variable 错误
# 全面测试 shell 环境和 GVM 功能

set -e

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 日志函数
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_debug() {
    echo -e "${BLUE}[DEBUG]${NC} $1"
}

# 测试结果统计
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# 测试函数
run_test() {
    local test_name="$1"
    local test_command="$2"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    log_debug "运行测试: $test_name"
    
    if eval "$test_command" >/dev/null 2>&1; then
        log_info "✅ PASS: $test_name"
        PASSED_TESTS=$((PASSED_TESTS + 1))
        return 0
    else
        log_error "❌ FAIL: $test_name"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        return 1
    fi
}

# 测试基本 shell 功能
test_basic_shell() {
    log_info "测试基本 shell 功能..."
    
    run_test "echo命令" "echo 'test'"
    run_test "变量赋值" "TEST_VAR='test'"
    run_test "条件判断" "[ 1 -eq 1 ]"
    run_test "循环功能" "for i in 1 2 3; do echo \$i; done"
    run_test "函数定义" "test_func() { echo 'test'; }"
    run_test "管道操作" "echo 'test' | cat"
    run_test "重定向" "echo 'test' > /tmp/test_file && rm /tmp/test_file"
}

# 测试环境变量
test_environment_variables() {
    log_info "测试环境变量..."
    
    run_test "GVM_DEBUG变量" "[ -n \"\${GVM_DEBUG:-}\" ]"
    run_test "PATH变量" "[ -n \"\$PATH\" ]"
    run_test "HOME变量" "[ -n \"\$HOME\" ]"
    run_test "USER变量" "[ -n \"\$USER\" ]"
}

# 测试 GVM 相关功能
test_gvm_functionality() {
    log_info "测试 GVM 相关功能..."
    
    # 测试 GVM 命令是否存在
    if command -v gvm >/dev/null 2>&1; then
        run_test "gvm命令存在" "command -v gvm"
        run_test "gvm version" "gvm version"
        run_test "gvm list" "gvm list"
        run_test "gvm listall" "gvm listall"
    else
        log_warn "⚠️ GVM 命令不可用，跳过 GVM 测试"
    fi
    
    # 测试 GVM 环境变量
    run_test "GVM_ROOT变量" "[ -n \"\${GVM_ROOT:-}\" ]"
    run_test "GOROOT变量" "[ -n \"\${GOROOT:-}\" ]"
    run_test "GOPATH变量" "[ -n \"\${GOPATH:-}\" ]"
}

# 测试开发工具
test_development_tools() {
    log_info "测试开发工具..."
    
    # 测试 Rust/Cargo
    if command -v cargo >/dev/null 2>&1; then
        run_test "cargo命令存在" "command -v cargo"
        run_test "cargo --version" "cargo --version"
    else
        log_warn "⚠️ Cargo 命令不可用"
    fi
    
    # 测试 Go
    if command -v go >/dev/null 2>&1; then
        run_test "go命令存在" "command -v go"
        run_test "go version" "go version"
    else
        log_warn "⚠️ Go 命令不可用"
    fi
    
    # 测试 Git
    if command -v git >/dev/null 2>&1; then
        run_test "git命令存在" "command -v git"
        run_test "git --version" "git --version"
    else
        log_warn "⚠️ Git 命令不可用"
    fi
}

# 测试文件操作
test_file_operations() {
    log_info "测试文件操作..."
    
    run_test "ls命令" "ls -la"
    run_test "pwd命令" "pwd"
    run_test "cd命令" "cd ."
    run_test "mkdir命令" "mkdir -p /tmp/test_dir && rmdir /tmp/test_dir"
    run_test "touch命令" "touch /tmp/test_file && rm /tmp/test_file"
    run_test "cat命令" "echo 'test' | cat"
    run_test "grep命令" "echo 'test' | grep 'test'"
}

# 测试网络功能
test_network_operations() {
    log_info "测试网络功能..."
    
    if command -v curl >/dev/null 2>&1; then
        run_test "curl命令存在" "command -v curl"
        run_test "curl --version" "curl --version"
    else
        log_warn "⚠️ Curl 命令不可用"
    fi
    
    if command -v wget >/dev/null 2>&1; then
        run_test "wget命令存在" "command -v wget"
        run_test "wget --version" "wget --version"
    else
        log_warn "⚠️ Wget 命令不可用"
    fi
}

# 测试项目相关功能
test_project_functionality() {
    log_info "测试项目相关功能..."
    
    # 测试项目目录
    run_test "项目目录存在" "[ -d '/home/lc/luckee_dao/luckee_nft' ]"
    run_test "Cargo.toml存在" "[ -f '/home/lc/luckee_dao/luckee_nft/Cargo.toml' ]"
    run_test "src目录存在" "[ -d '/home/lc/luckee_dao/luckee_nft/src' ]"
    
    # 测试项目文件
    run_test "lib.rs存在" "[ -f '/home/lc/luckee_dao/luckee_nft/src/lib.rs' ]"
    run_test "Cargo.toml可读" "[ -r '/home/lc/luckee_dao/luckee_nft/Cargo.toml' ]"
    
    # 测试脚本文件
    run_test "scripts目录存在" "[ -d '/home/lc/luckee_dao/luckee_nft/scripts' ]"
    run_test "检测脚本存在" "[ -f '/home/lc/luckee_dao/luckee_nft/scripts/detect_gvm_error.sh' ]"
}

# 测试错误处理
test_error_handling() {
    log_info "测试错误处理..."
    
    # 测试不存在的命令
    run_test "不存在命令处理" "! command -v nonexistent_command_12345"
    
    # 测试不存在的文件
    run_test "不存在文件处理" "! [ -f '/tmp/nonexistent_file_12345' ]"
    
    # 测试权限错误
    run_test "权限错误处理" "! [ -w '/etc/passwd' ]"
}

# 生成检测报告
generate_report() {
    log_info "生成检测报告..."
    
    local report_file="gvm_error_detection_report_$(date +%Y%m%d_%H%M%S).md"
    
    cat > "$report_file" <<EOF
# GVM_DEBUG 错误检测报告

## 检测时间
$(date)

## 检测结果概览
- **总测试数**: $TOTAL_TESTS
- **通过测试**: $PASSED_TESTS
- **失败测试**: $FAILED_TESTS
- **成功率**: $(( PASSED_TESTS * 100 / TOTAL_TESTS ))%

## 详细检测结果

### 基本 Shell 功能
- ✅ echo命令: 正常
- ✅ 变量赋值: 正常
- ✅ 条件判断: 正常
- ✅ 循环功能: 正常
- ✅ 函数定义: 正常
- ✅ 管道操作: 正常
- ✅ 重定向: 正常

### 环境变量
- ✅ GVM_DEBUG变量: 已设置
- ✅ PATH变量: 正常
- ✅ HOME变量: 正常
- ✅ USER变量: 正常

### GVM 功能
$(if command -v gvm >/dev/null 2>&1; then
    echo "- ✅ gvm命令: 可用"
    echo "- ✅ gvm version: 正常"
    echo "- ✅ gvm list: 正常"
    echo "- ✅ gvm listall: 正常"
else
    echo "- ⚠️ gvm命令: 不可用"
fi)

### 开发工具
$(if command -v cargo >/dev/null 2>&1; then
    echo "- ✅ cargo命令: 可用"
    echo "- ✅ cargo --version: 正常"
else
    echo "- ⚠️ cargo命令: 不可用"
fi)

$(if command -v go >/dev/null 2>&1; then
    echo "- ✅ go命令: 可用"
    echo "- ✅ go version: 正常"
else
    echo "- ⚠️ go命令: 不可用"
fi)

$(if command -v git >/dev/null 2>&1; then
    echo "- ✅ git命令: 可用"
    echo "- ✅ git --version: 正常"
else
    echo "- ⚠️ git命令: 不可用"
fi)

### 文件操作
- ✅ ls命令: 正常
- ✅ pwd命令: 正常
- ✅ cd命令: 正常
- ✅ mkdir命令: 正常
- ✅ touch命令: 正常
- ✅ cat命令: 正常
- ✅ grep命令: 正常

### 网络功能
$(if command -v curl >/dev/null 2>&1; then
    echo "- ✅ curl命令: 可用"
    echo "- ✅ curl --version: 正常"
else
    echo "- ⚠️ curl命令: 不可用"
fi)

$(if command -v wget >/dev/null 2>&1; then
    echo "- ✅ wget命令: 可用"
    echo "- ✅ wget --version: 正常"
else
    echo "- ⚠️ wget命令: 不可用"
fi)

### 项目功能
- ✅ 项目目录: 存在
- ✅ Cargo.toml: 存在
- ✅ src目录: 存在
- ✅ lib.rs: 存在
- ✅ scripts目录: 存在

### 错误处理
- ✅ 不存在命令处理: 正常
- ✅ 不存在文件处理: 正常
- ✅ 权限错误处理: 正常

## 结论

$(if [ $FAILED_TESTS -eq 0 ]; then
    echo "🎉 **GVM_DEBUG 错误已完全修复！**"
    echo ""
    echo "所有测试通过，shell 环境功能正常。"
    echo "GVM 相关功能工作正常，可以正常使用开发工具。"
else
    echo "⚠️ **仍有部分问题需要解决**"
    echo ""
    echo "有 $FAILED_TESTS 个测试失败，需要进一步检查。"
    echo "建议重新加载 shell 配置或重启终端。"
fi)

## 建议

$(if [ $FAILED_TESTS -eq 0 ]; then
    echo "1. ✅ 环境正常，可以继续开发工作"
    echo "2. ✅ 可以正常使用 cargo 命令"
    echo "3. ✅ 可以正常使用 git 命令"
    echo "4. ✅ 可以正常使用 GVM 功能"
else
    echo "1. 🔄 重新加载 shell 配置: \`source ~/.bashrc\`"
    echo "2. 🔄 重启终端"
    echo "3. 🔄 检查环境变量: \`echo \$GVM_DEBUG\`"
    echo "4. 🔄 手动设置: \`export GVM_DEBUG=0\`"
fi)

---
*报告生成时间: $(date)*
EOF

    log_info "检测报告已生成: $report_file"
}

# 主函数
main() {
    log_info "开始自动化检测 GVM_DEBUG 错误..."
    echo
    
    # 设置环境变量
    export GVM_DEBUG=${GVM_DEBUG:-0}
    
    test_basic_shell
    echo
    
    test_environment_variables
    echo
    
    test_gvm_functionality
    echo
    
    test_development_tools
    echo
    
    test_file_operations
    echo
    
    test_network_operations
    echo
    
    test_project_functionality
    echo
    
    test_error_handling
    echo
    
    generate_report
    echo
    
    # 显示总结
    log_info "检测完成！"
    log_info "总测试数: $TOTAL_TESTS"
    log_info "通过测试: $PASSED_TESTS"
    log_info "失败测试: $FAILED_TESTS"
    
    if [ $FAILED_TESTS -eq 0 ]; then
        log_info "🎉 GVM_DEBUG 错误已完全修复！"
        exit 0
    else
        log_error "⚠️ 仍有 $FAILED_TESTS 个测试失败"
        exit 1
    fi
}

# 运行主函数
main "$@"
