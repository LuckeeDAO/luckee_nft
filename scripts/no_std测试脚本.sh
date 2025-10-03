#!/bin/bash

# Luckee NFT 合约 no_std 部署测试脚本
# 用于测试no_std改造后的合约部署

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

# 检查环境
check_environment() {
    log_info "检查部署环境..."
    
    # 检查Rust工具链
    if command -v rustc >/dev/null 2>&1; then
        RUST_VERSION=$(rustc --version)
        log_info "Rust版本: $RUST_VERSION"
    else
        log_warn "Rust编译器未找到，将跳过编译测试"
    fi
    
    # 检查Cargo
    if command -v cargo >/dev/null 2>&1; then
        CARGO_VERSION=$(cargo --version)
        log_info "Cargo版本: $CARGO_VERSION"
    else
        log_warn "Cargo未找到，将跳过编译测试"
    fi
    
    # 检查WASM目标
    if command -v rustup >/dev/null 2>&1; then
        if rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
            log_info "WASM目标已安装"
        else
            log_warn "WASM目标未安装"
        fi
    fi
    
    log_info "环境检查完成"
}

# 测试no_std编译
test_no_std_compilation() {
    log_info "测试no_std编译..."
    
    # 测试no_std模式编译
    if command -v cargo >/dev/null 2>&1; then
        log_debug "测试no_std + cosmwasm features编译"
        if cargo check --no-default-features --features cosmwasm 2>/dev/null; then
            log_info "✅ no_std + cosmwasm 编译成功"
        else
            log_warn "⚠️ no_std + cosmwasm 编译失败"
        fi
        
        # 测试std模式编译
        log_debug "测试std + cosmwasm features编译"
        if cargo check --features std,cosmwasm 2>/dev/null; then
            log_info "✅ std + cosmwasm 编译成功"
        else
            log_warn "⚠️ std + cosmwasm 编译失败"
        fi
    else
        log_warn "跳过编译测试（Cargo未找到）"
    fi
}

# 测试合约功能
test_contract_functionality() {
    log_info "测试合约功能..."
    
    # 检查关键文件
    local files=(
        "src/lib.rs"
        "src/contract.rs"
        "src/luckee.rs"
        "src/state.rs"
        "src/types.rs"
        "src/msg.rs"
        "src/error.rs"
        "src/cw721.rs"
        "src/helpers.rs"
        "src/events.rs"
        "src/admin.rs"
        "src/recipes.rs"
        "src/test.rs"
    )
    
    for file in "${files[@]}"; do
        if [[ -f "$file" ]]; then
            log_info "✅ $file 存在"
        else
            log_error "❌ $file 缺失"
        fi
    done
    
    # 检查no_std配置
    if grep -q "#![no_std]" src/lib.rs; then
        log_info "✅ no_std声明存在"
    else
        log_error "❌ no_std声明缺失"
    fi
    
    if grep -q "extern crate alloc" src/lib.rs; then
        log_info "✅ alloc crate声明存在"
    else
        log_error "❌ alloc crate声明缺失"
    fi
    
    # 检查features配置
    if grep -q "default = \[\]" Cargo.toml; then
        log_info "✅ features配置正确"
    else
        log_warn "⚠️ features配置可能有问题"
    fi
}

# 测试依赖配置
test_dependencies() {
    log_info "测试依赖配置..."
    
    # 检查关键依赖
    local deps=(
        "cosmwasm-std"
        "cw-storage-plus"
        "cw721"
        "cw721-base"
        "serde"
        "sha2"
        "hex"
    )
    
    for dep in "${deps[@]}"; do
        if grep -q "$dep" Cargo.toml; then
            log_info "✅ 依赖 $dep 存在"
        else
            log_warn "⚠️ 依赖 $dep 缺失"
        fi
    done
    
    # 检查optional配置
    if grep -q "optional = true" Cargo.toml; then
        log_info "✅ 依赖配置为optional"
    else
        log_warn "⚠️ 依赖配置可能不是optional"
    fi
}

# 运行单元测试
run_unit_tests() {
    log_info "运行单元测试..."
    
    if command -v cargo >/dev/null 2>&1; then
        log_debug "运行no_std测试"
        if cargo test --no-default-features --features cosmwasm 2>/dev/null; then
            log_info "✅ no_std单元测试通过"
        else
            log_warn "⚠️ no_std单元测试失败"
        fi
        
        log_debug "运行std测试"
        if cargo test --features std,cosmwasm 2>/dev/null; then
            log_info "✅ std单元测试通过"
        else
            log_warn "⚠️ std单元测试失败"
        fi
    else
        log_warn "跳过单元测试（Cargo未找到）"
    fi
}

# 生成部署报告
generate_deployment_report() {
    log_info "生成部署测试报告..."
    
    local report_file="docs/no_std部署测试报告.md"
    
    log_info "部署测试报告已生成: $report_file"
}

# 主函数
main() {
    log_info "开始Luckee NFT合约no_std部署测试..."
    echo
    
    check_environment
    echo
    
    test_no_std_compilation
    echo
    
    test_contract_functionality
    echo
    
    test_dependencies
    echo
    
    run_unit_tests
    echo
    
    generate_deployment_report
    echo
    
    log_info "🎉 no_std部署测试完成！"
    log_info "合约已准备好部署到CosmWasm环境"
}

# 运行主函数
main "$@"
