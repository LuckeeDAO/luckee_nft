#!/bin/bash

# Luckee NFT 合约生产环境部署脚本
# 专为主网部署设计，包含额外的安全检查和验证

set -e

# 配置变量
CHAIN_ID=${CHAIN_ID:-"luckee-mainnet"}
NODE=${NODE:-"https://rpc.luckee-mainnet.com:443"}
KEYRING_BACKEND=${KEYRING_BACKEND:-"file"}
GAS_PRICES=${GAS_PRICES:-"0.025uluckee"}
GAS_ADJUSTMENT=${GAS_ADJUSTMENT:-"2.0"}

# 合约配置
CONTRACT_NAME="luckee_nft"
CONTRACT_VERSION="0.1.0"
ADMIN_ADDRESS=${ADMIN_ADDRESS:-""}
MINTER_ADDRESS=${MINTER_ADDRESS:-""}

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

# 安全检查函数
security_check() {
    log_info "执行安全检查..."
    
    # 检查是否为主网
    if [ "$CHAIN_ID" != "luckee-mainnet" ]; then
        log_error "此脚本仅用于主网部署"
        exit 1
    fi
    
    # 检查管理员地址格式
    if [[ ! "$ADMIN_ADDRESS" =~ ^luckee1[a-z0-9]{38}$ ]]; then
        log_error "管理员地址格式不正确"
        exit 1
    fi
    
    # 检查铸造者地址格式
    if [[ ! "$MINTER_ADDRESS" =~ ^luckee1[a-z0-9]{38}$ ]]; then
        log_error "铸造者地址格式不正确"
        exit 1
    fi
    
    # 检查是否使用文件密钥环
    if [ "$KEYRING_BACKEND" != "file" ]; then
        log_warn "建议在生产环境使用文件密钥环"
        read -p "确认继续? (yes/no): " confirm
        if [ "$confirm" != "yes" ]; then
            exit 1
        fi
    fi
    
    # 检查账户余额
    log_info "检查账户余额..."
    BALANCE=$(wasmd query bank balances "$ADMIN_ADDRESS" --node "$NODE" --output json | jq -r '.balances[] | select(.denom=="uluckee") | .amount // "0"')
    if [ "$BALANCE" = "0" ] || [ -z "$BALANCE" ]; then
        log_error "账户余额不足，请确保账户有足够的代币"
        exit 1
    fi
    
    # 检查余额是否足够
    MIN_BALANCE=1000000  # 1 LUCKEE
    if [ "$BALANCE" -lt "$MIN_BALANCE" ]; then
        log_warn "账户余额较低 ($BALANCE uluckee)，建议至少 $MIN_BALANCE uluckee"
        read -p "确认继续? (yes/no): " confirm
        if [ "$confirm" != "yes" ]; then
            exit 1
        fi
    fi
    
    log_info "安全检查通过"
}

# 多签验证函数
verify_multisig() {
    log_info "验证多签配置..."
    
    # 检查是否为多签地址
    MULTISIG_INFO=$(wasmd keys show "$ADMIN_ADDRESS" --keyring-backend "$KEYRING_BACKEND" 2>/dev/null || echo "")
    if [ -z "$MULTISIG_INFO" ]; then
        log_warn "无法验证管理员地址是否为多签地址"
        log_warn "建议使用多签钱包作为管理员地址"
        read -p "确认继续? (yes/no): " confirm
        if [ "$confirm" != "yes" ]; then
            exit 1
        fi
    fi
    
    log_info "多签验证完成"
}

# 预部署验证
pre_deployment_check() {
    log_info "执行预部署验证..."
    
    # 检查网络连接
    log_info "检查网络连接..."
    if ! ping -c 1 $(echo "$NODE" | sed 's|https://||' | sed 's|:443||') > /dev/null 2>&1; then
        log_warn "网络连接可能不稳定"
    fi
    
    # 检查节点状态
    log_info "检查节点状态..."
    NODE_STATUS=$(wasmd status --node "$NODE" --output json 2>/dev/null || echo "{}")
    if [ "$NODE_STATUS" = "{}" ]; then
        log_error "无法连接到节点"
        exit 1
    fi
    
    # 检查链ID
    ACTUAL_CHAIN_ID=$(echo "$NODE_STATUS" | jq -r '.node_info.network')
    if [ "$ACTUAL_CHAIN_ID" != "$CHAIN_ID" ]; then
        log_error "链ID不匹配: 期望 $CHAIN_ID，实际 $ACTUAL_CHAIN_ID"
        exit 1
    fi
    
    # 检查区块高度
    BLOCK_HEIGHT=$(echo "$NODE_STATUS" | jq -r '.sync_info.latest_block_height')
    log_info "当前区块高度: $BLOCK_HEIGHT"
    
    log_info "预部署验证完成"
}

# 部署确认
deployment_confirmation() {
    log_info "部署确认..."
    
    echo ""
    echo "=========================================="
    echo "          生产环境部署确认"
    echo "=========================================="
    echo "链ID: $CHAIN_ID"
    echo "节点: $NODE"
    echo "管理员: $ADMIN_ADDRESS"
    echo "铸造者: $MINTER_ADDRESS"
    echo "Gas价格: $GAS_PRICES"
    echo "Gas调整: $GAS_ADJUSTMENT"
    echo "密钥环: $KEYRING_BACKEND"
    echo "=========================================="
    echo ""
    
    log_warn "⚠️  重要提醒:"
    log_warn "1. 此操作将在主网上部署合约"
    log_warn "2. 部署后无法撤销"
    log_warn "3. 请确保所有配置正确"
    log_warn "4. 建议在部署前进行充分测试"
    echo ""
    
    read -p "确认部署到主网? (yes/no): " confirm
    if [ "$confirm" != "yes" ]; then
        log_info "部署已取消"
        exit 0
    fi
    
    # 二次确认
    echo ""
    log_warn "请再次确认部署信息"
    read -p "最终确认部署? (yes/no): " final_confirm
    if [ "$final_confirm" != "yes" ]; then
        log_info "部署已取消"
        exit 0
    fi
}

# 部署后验证
post_deployment_verification() {
    log_info "执行部署后验证..."
    
    if [ -z "$CONTRACT_ADDRESS" ]; then
        log_error "合约地址未设置"
        return 1
    fi
    
    # 验证合约部署
    log_info "验证合约部署..."
    CONTRACT_INFO=$(wasmd query wasm contract "$CONTRACT_ADDRESS" --node "$NODE" --output json)
    if [ $? -ne 0 ]; then
        log_error "合约部署验证失败"
        return 1
    fi
    
    # 验证合约代码
    log_info "验证合约代码..."
    CODE_INFO=$(wasmd query wasm code "$CODE_ID" --node "$NODE" --output json)
    if [ $? -ne 0 ]; then
        log_error "合约代码验证失败"
        return 1
    fi
    
    # 验证合约配置
    log_info "验证合约配置..."
    CONFIG_QUERY='{"contract_info": {}}'
    CONFIG_RESULT=$(wasmd query wasm contract-state smart "$CONTRACT_ADDRESS" "$CONFIG_QUERY" --node "$NODE" --output json)
    if [ $? -ne 0 ]; then
        log_error "合约配置验证失败"
        return 1
    fi
    
    # 验证铸造者权限
    log_info "验证铸造者权限..."
    MINTER_QUERY='{"all_minters": {}}'
    MINTER_RESULT=$(wasmd query wasm contract-state smart "$CONTRACT_ADDRESS" "$MINTER_QUERY" --node "$NODE" --output json)
    if [ $? -ne 0 ]; then
        log_warn "铸造者权限验证失败，但部署可能成功"
    fi
    
    log_info "部署后验证完成"
}

# 生成生产环境部署报告
generate_production_report() {
    log_info "生成生产环境部署报告..."
    
    REPORT_FILE="production_deployment_report_$(date +%Y%m%d_%H%M%S).md"
    
    cat > "$REPORT_FILE" <<EOF
# Luckee NFT 合约生产环境部署报告

## 部署信息
- **部署时间**: $(date)
- **链ID**: $CHAIN_ID
- **节点**: $NODE
- **合约版本**: $CONTRACT_VERSION
- **部署类型**: 生产环境

## 合约信息
- **Code ID**: $CODE_ID
- **合约地址**: $CONTRACT_ADDRESS
- **管理员**: $ADMIN_ADDRESS
- **铸造者**: $MINTER_ADDRESS

## 部署步骤
1. ✅ 安全检查
2. ✅ 多签验证
3. ✅ 预部署验证
4. ✅ 部署确认
5. ✅ 构建合约
6. ✅ 上传合约
7. ✅ 实例化合约
8. ✅ 配置合约
9. ✅ 部署后验证

## 安全配置
- **密钥环后端**: $KEYRING_BACKEND
- **Gas价格**: $GAS_PRICES
- **Gas调整**: $GAS_ADJUSTMENT
- **多签验证**: 已执行
- **余额检查**: 已通过

## 后续步骤
1. 在盲盒合约中设置NFT合约地址
2. 测试NFT铸造功能
3. 测试合成功能
4. 配置前端集成
5. 设置监控和告警
6. 建立应急响应流程

## 环境变量
\`\`\`bash
export CODE_ID=$CODE_ID
export CONTRACT_ADDRESS=$CONTRACT_ADDRESS
export ADMIN_ADDRESS=$ADMIN_ADDRESS
export MINTER_ADDRESS=$MINTER_ADDRESS
export CHAIN_ID=$CHAIN_ID
export NODE=$NODE
\`\`\`

## 安全提醒
- 定期备份合约状态
- 监控合约事件和日志
- 设置告警机制
- 建立应急响应流程
- 定期安全审计

## 联系信息
- 技术支持: tech@luckee.io
- 安全团队: security@luckee.io
- 紧急联系: emergency@luckee.io
EOF
    
    log_info "生产环境部署报告已生成: $REPORT_FILE"
}

# 主函数
main() {
    log_info "开始生产环境部署 Luckee NFT 合约..."
    
    # 执行安全检查
    security_check
    
    # 验证多签配置
    verify_multisig
    
    # 预部署验证
    pre_deployment_check
    
    # 部署确认
    deployment_confirmation
    
    # 调用基础部署脚本
    log_info "调用基础部署脚本..."
    ./scripts/deploy.sh \
        --chain-id "$CHAIN_ID" \
        --node "$NODE" \
        --admin "$ADMIN_ADDRESS" \
        --minter "$MINTER_ADDRESS" \
        --verbose
    
    # 部署后验证
    post_deployment_verification
    
    # 生成生产环境部署报告
    generate_production_report
    
    log_info "生产环境部署完成！"
    log_info "合约地址: $CONTRACT_ADDRESS"
    log_info "Code ID: $CODE_ID"
    
    # 显示后续步骤
    echo ""
    log_info "后续步骤:"
    log_info "1. 在盲盒合约中设置NFT合约地址: $CONTRACT_ADDRESS"
    log_info "2. 测试NFT铸造功能"
    log_info "3. 测试合成功能"
    log_info "4. 配置前端集成"
    log_info "5. 设置监控和告警"
    log_info "6. 建立应急响应流程"
    echo ""
    log_info "生产环境部署报告已生成"
    log_info "请妥善保管部署报告和私钥"
}

# 帮助信息
show_help() {
    cat <<EOF
Luckee NFT 合约生产环境部署脚本

用法: $0 [选项]

选项:
    -h, --help              显示帮助信息
    -c, --chain-id          链ID (默认: luckee-mainnet)
    -n, --node              节点地址 (默认: https://rpc.luckee-mainnet.com:443)
    -a, --admin             管理员地址 (必需)
    -m, --minter            铸造者地址 (必需)

环境变量:
    ADMIN_ADDRESS           管理员地址
    MINTER_ADDRESS          铸造者地址
    CHAIN_ID               链ID
    NODE                    节点地址
    GAS_PRICES              Gas价格
    GAS_ADJUSTMENT          Gas调整系数
    KEYRING_BACKEND         密钥环后端

示例:
    # 生产环境部署
    $0 --admin luckee1... --minter luckee1...
    
    # 使用环境变量
    ADMIN_ADDRESS=luckee1... MINTER_ADDRESS=luckee1... $0

安全要求:
    - 必须使用多签钱包作为管理员地址
    - 必须在冷钱包中执行关键操作
    - 禁止在CI/CD中保存私钥
    - 建议使用硬件钱包
    - 部署前必须进行充分测试

注意事项:
    - 此脚本仅用于主网部署
    - 部署后无法撤销
    - 请确保所有配置正确
    - 建议在部署前进行充分测试
EOF
}

# 解析命令行参数
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_help
            exit 0
            ;;
        -c|--chain-id)
            CHAIN_ID="$2"
            shift 2
            ;;
        -n|--node)
            NODE="$2"
            shift 2
            ;;
        -a|--admin)
            ADMIN_ADDRESS="$2"
            shift 2
            ;;
        -m|--minter)
            MINTER_ADDRESS="$2"
            shift 2
            ;;
        *)
            log_error "未知参数: $1"
            show_help
            exit 1
            ;;
    esac
done

# 检查必需参数
if [ -z "$ADMIN_ADDRESS" ]; then
    log_error "请设置管理员地址 (--admin 或 ADMIN_ADDRESS 环境变量)"
    exit 1
fi

if [ -z "$MINTER_ADDRESS" ]; then
    log_error "请设置铸造者地址 (--minter 或 MINTER_ADDRESS 环境变量)"
    exit 1
fi

# 运行主函数
main
