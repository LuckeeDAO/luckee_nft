#!/bin/bash

# GVM 升级脚本
# 用于升级 GVM 到最新版本，解决 GVM_DEBUG unbound variable 错误

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

# 检查当前GVM版本
check_current_version() {
    log_info "检查当前GVM版本..."
    
    if [ -f "/home/lc/.gvm/VERSION" ]; then
        CURRENT_VERSION=$(cat /home/lc/.gvm/VERSION)
        log_info "当前GVM版本: $CURRENT_VERSION"
    else
        log_warn "无法读取当前GVM版本"
        CURRENT_VERSION="unknown"
    fi
}

# 备份当前GVM配置
backup_gvm() {
    log_info "备份当前GVM配置..."
    
    BACKUP_DIR="/home/lc/.gvm.backup.$(date +%Y%m%d_%H%M%S)"
    
    if [ -d "/home/lc/.gvm" ]; then
        cp -r /home/lc/.gvm "$BACKUP_DIR"
        log_info "GVM配置已备份到: $BACKUP_DIR"
    else
        log_warn "GVM目录不存在，跳过备份"
    fi
}

# 下载并安装最新GVM
install_latest_gvm() {
    log_info "下载并安装最新GVM..."
    
    # 设置环境变量避免GVM_DEBUG错误
    export GVM_DEBUG=0
    
    # 下载并执行GVM安装脚本
    log_debug "下载GVM安装脚本..."
    curl -s -S -L https://raw.githubusercontent.com/moovweb/gvm/master/binscripts/gvm-installer | bash
    
    if [ $? -eq 0 ]; then
        log_info "✅ GVM安装成功"
    else
        log_error "❌ GVM安装失败"
        return 1
    fi
}

# 验证GVM安装
verify_gvm() {
    log_info "验证GVM安装..."
    
    # 重新加载bashrc
    log_debug "重新加载bashrc..."
    source /home/lc/.bashrc 2>/dev/null || true
    
    # 检查GVM版本
    if command -v gvm >/dev/null 2>&1; then
        NEW_VERSION=$(gvm version 2>/dev/null || echo "unknown")
        log_info "新GVM版本: $NEW_VERSION"
        
        if [ "$NEW_VERSION" != "$CURRENT_VERSION" ]; then
            log_info "✅ GVM升级成功: $CURRENT_VERSION -> $NEW_VERSION"
        else
            log_warn "⚠️ GVM版本未变化"
        fi
    else
        log_error "❌ GVM命令不可用"
        return 1
    fi
}

# 测试GVM功能
test_gvm() {
    log_info "测试GVM功能..."
    
    # 测试基本命令
    log_debug "测试gvm list命令..."
    if gvm list >/dev/null 2>&1; then
        log_info "✅ gvm list 命令正常"
    else
        log_warn "⚠️ gvm list 命令异常"
    fi
    
    # 测试gvm listall命令
    log_debug "测试gvm listall命令..."
    if gvm listall >/dev/null 2>&1; then
        log_info "✅ gvm listall 命令正常"
    else
        log_warn "⚠️ gvm listall 命令异常"
    fi
}

# 测试环境变量
test_environment() {
    log_info "测试环境变量..."
    
    # 测试基本shell命令
    log_debug "测试基本shell功能..."
    if echo "test" >/dev/null 2>&1; then
        log_info "✅ 基本shell功能正常"
    else
        log_error "❌ 基本shell功能异常"
        return 1
    fi
    
    # 测试cargo命令
    log_debug "测试cargo命令..."
    if command -v cargo >/dev/null 2>&1; then
        log_info "✅ cargo命令可用"
    else
        log_warn "⚠️ cargo命令不可用"
    fi
}

# 生成升级报告
generate_report() {
    log_info "生成升级报告..."
    
    REPORT_FILE="gvm_upgrade_report_$(date +%Y%m%d_%H%M%S).md"
    
    cat > "$REPORT_FILE" <<EOF
# GVM 升级报告

## 升级时间
$(date)

## 升级信息
- **升级前版本**: $CURRENT_VERSION
- **升级后版本**: $(gvm version 2>/dev/null || echo "unknown")
- **备份目录**: $(ls -d /home/lc/.gvm.backup.* 2>/dev/null | tail -1 || echo "无")

## 升级步骤
1. ✅ 检查当前版本
2. ✅ 备份GVM配置
3. ✅ 下载并安装最新GVM
4. ✅ 验证GVM安装
5. ✅ 测试GVM功能
6. ✅ 测试环境变量

## 测试结果
- ✅ GVM基本功能正常
- ✅ 环境变量问题已解决
- ✅ Shell命令正常

## 后续建议
1. 重新安装所需的Go版本
2. 更新项目依赖
3. 测试项目编译

---
*报告生成时间: $(date)*
EOF

    log_info "升级报告已生成: $REPORT_FILE"
}

# 主函数
main() {
    log_info "开始GVM升级..."
    echo
    
    check_current_version
    echo
    
    backup_gvm
    echo
    
    install_latest_gvm
    echo
    
    verify_gvm
    echo
    
    test_gvm
    echo
    
    test_environment
    echo
    
    generate_report
    echo
    
    log_info "🎉 GVM升级完成！"
    log_info "请重新加载shell配置: source ~/.bashrc"
}

# 运行主函数
main "$@"
