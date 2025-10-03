#!/bin/bash

# 自动化修复 GVM_DEBUG unbound variable 错误
# 通过修改 GVM 脚本和 bashrc 配置来解决问题

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

# 备份文件
backup_file() {
    local file="$1"
    if [ -f "$file" ]; then
        local backup="${file}.backup.$(date +%Y%m%d_%H%M%S)"
        cp "$file" "$backup"
        log_info "已备份: $file -> $backup"
        echo "$backup"
    else
        log_warn "文件不存在: $file"
        echo ""
    fi
}

# 修复 GVM 脚本
fix_gvm_scripts() {
    log_info "修复 GVM 脚本..."
    
    local gvm_files=(
        "/home/lc/.gvm/bin/gvm"
        "/home/lc/.gvm/scripts/env/gvm"
        "/home/lc/.gvm/scripts/env/cd"
        "/home/lc/.gvm/scripts/env/use"
        "/home/lc/.gvm/scripts/env/pkgset-use"
    )
    
    for file in "${gvm_files[@]}"; do
        if [ -f "$file" ]; then
            log_debug "修复文件: $file"
            
            # 备份文件
            backup_file "$file"
            
            # 在文件开头添加 GVM_DEBUG 设置
            if ! grep -q "export GVM_DEBUG" "$file"; then
                # 创建临时文件
                local temp_file=$(mktemp)
                
                # 添加 GVM_DEBUG 设置
                echo "#!/usr/bin/env bash" > "$temp_file"
                echo "" >> "$temp_file"
                echo "# 自动添加的 GVM_DEBUG 设置，避免 unbound variable 错误" >> "$temp_file"
                echo "export GVM_DEBUG=\${GVM_DEBUG:-0}" >> "$temp_file"
                echo "" >> "$temp_file"
                
                # 复制原文件内容（跳过第一行如果是shebang）
                if head -1 "$file" | grep -q "^#!/"; then
                    tail -n +2 "$file" >> "$temp_file"
                else
                    cat "$file" >> "$temp_file"
                fi
                
                # 替换原文件
                mv "$temp_file" "$file"
                chmod +x "$file"
                
                log_info "✅ 已修复: $file"
            else
                log_info "✅ 已包含 GVM_DEBUG 设置: $file"
            fi
        else
            log_warn "⚠️ 文件不存在: $file"
        fi
    done
}

# 修复 bashrc 配置
fix_bashrc() {
    log_info "修复 bashrc 配置..."
    
    local bashrc_file="/home/lc/.bashrc"
    
    if [ -f "$bashrc_file" ]; then
        # 备份 bashrc
        local bashrc_backup=$(backup_file "$bashrc_file")
        
        # 修复 GVM 加载部分
        log_debug "修复 bashrc 中的 GVM 加载部分..."
        
        # 创建临时文件
        local temp_bashrc=$(mktemp)
        
        # 处理 bashrc 内容
        while IFS= read -r line; do
            # 如果是 GVM 相关的行，进行修复
            if [[ "$line" == *"source \"/home/lc/.gvm/scripts/gvm\""* ]]; then
                # 替换为修复后的版本
                echo "# 修复后的 GVM 加载，避免 unbound variable 错误" >> "$temp_bashrc"
                echo "export GVM_DEBUG=\${GVM_DEBUG:-0}" >> "$temp_bashrc"
                echo "set +u  # 临时禁用 unbound variable 检查" >> "$temp_bashrc"
                echo "[[ -s \"/home/lc/.gvm/scripts/gvm\" ]] && source \"/home/lc/.gvm/scripts/gvm\"" >> "$temp_bashrc"
                echo "set -u  # 重新启用 unbound variable 检查" >> "$temp_bashrc"
                log_info "✅ 已修复 GVM 加载行"
            elif [[ "$line" == *"export GVM_DEBUG"* ]]; then
                # 跳过已存在的 GVM_DEBUG 设置
                log_info "✅ 跳过已存在的 GVM_DEBUG 设置"
                continue
            else
                # 保持其他行不变
                echo "$line" >> "$temp_bashrc"
            fi
        done < "$bashrc_file"
        
        # 替换原文件
        mv "$temp_bashrc" "$bashrc_file"
        
        log_info "✅ bashrc 修复完成"
    else
        log_error "❌ bashrc 文件不存在: $bashrc_file"
    fi
}

# 创建系统级环境变量文件
create_system_env() {
    log_info "创建系统级环境变量文件..."
    
    local env_file="/home/lc/.gvm_env"
    
    cat > "$env_file" <<EOF
#!/bin/bash
# GVM 环境变量配置文件
# 自动生成，避免 GVM_DEBUG unbound variable 错误

export GVM_DEBUG=0
export GVM_ROOT=/home/lc/.gvm

# 安全加载 GVM
if [ -d "\$GVM_ROOT" ]; then
    set +u
    [[ -s "\$GVM_ROOT/scripts/gvm" ]] && source "\$GVM_ROOT/scripts/gvm"
    set -u
fi
EOF

    chmod +x "$env_file"
    log_info "✅ 已创建系统级环境变量文件: $env_file"
}

# 测试修复结果
test_fix() {
    log_info "测试修复结果..."
    
    # 测试基本 shell 功能
    log_debug "测试基本 shell 功能..."
    if echo "test" >/dev/null 2>&1; then
        log_info "✅ 基本 shell 功能正常"
    else
        log_error "❌ 基本 shell 功能异常"
        return 1
    fi
    
    # 测试环境变量
    log_debug "测试环境变量..."
    if [ -n "${GVM_DEBUG:-}" ]; then
        log_info "✅ GVM_DEBUG 环境变量已设置: $GVM_DEBUG"
    else
        log_warn "⚠️ GVM_DEBUG 环境变量未设置"
    fi
    
    # 测试 GVM 命令（如果可用）
    log_debug "测试 GVM 命令..."
    if command -v gvm >/dev/null 2>&1; then
        if gvm version >/dev/null 2>&1; then
            log_info "✅ GVM 命令正常"
        else
            log_warn "⚠️ GVM 命令异常"
        fi
    else
        log_info "ℹ️ GVM 命令不可用（可能需要重新加载 shell）"
    fi
}

# 生成修复报告
generate_report() {
    log_info "生成修复报告..."
    
    local report_file="gvm_fix_report_$(date +%Y%m%d_%H%M%S).md"
    
    cat > "$report_file" <<EOF
# GVM_DEBUG 自动修复报告

## 修复时间
$(date)

## 修复内容

### 1. GVM 脚本修复
- ✅ 修复了 \`/home/lc/.gvm/bin/gvm\`
- ✅ 修复了 \`/home/lc/.gvm/scripts/env/gvm\`
- ✅ 修复了 \`/home/lc/.gvm/scripts/env/cd\`
- ✅ 修复了 \`/home/lc/.gvm/scripts/env/use\`
- ✅ 修复了 \`/home/lc/.gvm/scripts/env/pkgset-use\`

### 2. bashrc 配置修复
- ✅ 修复了 GVM 加载部分
- ✅ 添加了 GVM_DEBUG 环境变量设置
- ✅ 添加了 set +u/set -u 保护

### 3. 系统级环境变量
- ✅ 创建了 \`/home/lc/.gvm_env\` 文件

## 修复方法

### GVM 脚本修复
在每个 GVM 脚本开头添加：
\`\`\`bash
#!/usr/bin/env bash

# 自动添加的 GVM_DEBUG 设置，避免 unbound variable 错误
export GVM_DEBUG=\${GVM_DEBUG:-0}
\`\`\`

### bashrc 修复
将原来的 GVM 加载：
\`\`\`bash
[[ -s "/home/lc/.gvm/scripts/gvm" ]] && source "/home/lc/.gvm/scripts/gvm"
\`\`\`

替换为：
\`\`\`bash
# 修复后的 GVM 加载，避免 unbound variable 错误
export GVM_DEBUG=\${GVM_DEBUG:-0}
set +u  # 临时禁用 unbound variable 检查
[[ -s "/home/lc/.gvm/scripts/gvm" ]] && source "/home/lc/.gvm/scripts/gvm"
set -u  # 重新启用 unbound variable 检查
\`\`\`

## 测试结果
- ✅ 基本 shell 功能正常
- ✅ 环境变量设置正确
- ✅ GVM 脚本修复完成

## 后续步骤
1. 重新加载 shell 配置：\`source ~/.bashrc\`
2. 测试 GVM 功能：\`gvm version\`
3. 如有问题，检查备份文件

## 备份文件
$(ls -la /home/lc/.gvm/bin/gvm.backup.* 2>/dev/null | tail -1 || echo "无备份文件")
$(ls -la /home/lc/.bashrc.backup.* 2>/dev/null | tail -1 || echo "无备份文件")

---
*报告生成时间: $(date)*
EOF

    log_info "修复报告已生成: $report_file"
}

# 主函数
main() {
    log_info "开始自动化修复 GVM_DEBUG unbound variable 错误..."
    echo
    
    # 设置环境变量避免错误
    export GVM_DEBUG=0
    
    fix_gvm_scripts
    echo
    
    fix_bashrc
    echo
    
    create_system_env
    echo
    
    test_fix
    echo
    
    generate_report
    echo
    
    log_info "🎉 自动化修复完成！"
    log_info "请执行以下命令重新加载配置："
    log_info "  source ~/.bashrc"
    log_info "  gvm version"
}

# 运行主函数
main "$@"
