#!/bin/bash

# Luckee NFT 合约部署脚本
# 用于部署到测试网或主网

set -e

# 配置变量
CHAIN_ID=${CHAIN_ID:-"luckee-testnet"}
NODE=${NODE:-"https://rpc.luckee-testnet.com:443"}
KEYRING_BACKEND=${KEYRING_BACKEND:-"test"}
GAS_PRICES=${GAS_PRICES:-"0.025uluckee"}
GAS_ADJUSTMENT=${GAS_ADJUSTMENT:-"1.5"}

# 合约配置
CONTRACT_NAME="luckee_nft"
CONTRACT_VERSION="0.1.0"
ADMIN_ADDRESS=${ADMIN_ADDRESS:-""}
MINTER_ADDRESS=${MINTER_ADDRESS:-""}

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
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

# 检查依赖
check_dependencies() {
    log_info "检查依赖..."
    
    if ! command -v cargo &> /dev/null; then
        log_error "cargo 未安装"
        exit 1
    fi
    
    if ! command -v wasmd &> /dev/null; then
        log_error "wasmd 未安装"
        exit 1
    fi
    
    log_info "依赖检查完成"
}

# 构建合约
build_contract() {
    if [ "$SKIP_BUILD" = true ]; then
        log_info "跳过构建步骤"
        return 0
    fi
    
    log_info "构建合约..."
    
    cd "$(dirname "$0")/.."
    
    # 检查WASM文件是否已存在
    if [ -f "target/wasm32-unknown-unknown/release/luckee_nft_optimized.wasm" ] && [ "$DRY_RUN" = false ]; then
        log_warn "WASM文件已存在，是否重新构建？"
        read -p "重新构建? (y/N): " rebuild
        if [ "$rebuild" != "y" ] && [ "$rebuild" != "Y" ]; then
            log_info "使用现有WASM文件"
            return 0
        fi
    fi
    
    if [ "$DRY_RUN" = true ]; then
        log_info "[DRY RUN] 将执行以下命令:"
        echo "RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown"
        echo "wasm-opt -Os target/wasm32-unknown-unknown/release/luckee_nft.wasm -o target/wasm32-unknown-unknown/release/luckee_nft_optimized.wasm"
        return 0
    fi
    
    # 优化构建
    if [ "$VERBOSE" = true ]; then
        RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown --verbose
    else
        RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown
    fi
    
    # 检查wasm-opt是否可用
    if command -v wasm-opt &> /dev/null; then
        log_info "优化WASM文件..."
        wasm-opt -Os target/wasm32-unknown-unknown/release/luckee_nft.wasm -o target/wasm32-unknown-unknown/release/luckee_nft_optimized.wasm
    else
        log_warn "wasm-opt 未找到，跳过优化步骤"
        cp target/wasm32-unknown-unknown/release/luckee_nft.wasm target/wasm32-unknown-unknown/release/luckee_nft_optimized.wasm
    fi
    
    # 显示文件信息
    log_info "WASM文件信息:"
    ls -lh target/wasm32-unknown-unknown/release/luckee_nft_optimized.wasm
    
    # 计算文件哈希
    if command -v sha256sum &> /dev/null; then
        WASM_HASH=$(sha256sum target/wasm32-unknown-unknown/release/luckee_nft_optimized.wasm | cut -d' ' -f1)
        log_info "WASM文件哈希: $WASM_HASH"
    fi
    
    log_info "合约构建完成"
}

# 上传合约
upload_contract() {
    if [ "$SKIP_UPLOAD" = true ]; then
        log_info "跳过上传步骤"
        return 0
    fi
    
    log_info "上传合约到链上..."
    
    if [ -z "$ADMIN_ADDRESS" ]; then
        log_error "请设置 ADMIN_ADDRESS 环境变量"
        exit 1
    fi
    
    # 检查WASM文件是否存在
    if [ ! -f "target/wasm32-unknown-unknown/release/luckee_nft_optimized.wasm" ]; then
        log_error "WASM文件不存在，请先构建合约"
        exit 1
    fi
    
    if [ "$DRY_RUN" = true ]; then
        log_info "[DRY RUN] 将执行以下命令:"
        echo "wasmd tx wasm store target/wasm32-unknown-unknown/release/luckee_nft_optimized.wasm \\"
        echo "  --from \"$ADMIN_ADDRESS\" \\"
        echo "  --chain-id \"$CHAIN_ID\" \\"
        echo "  --node \"$NODE\" \\"
        echo "  --keyring-backend \"$KEYRING_BACKEND\" \\"
        echo "  --gas-prices \"$GAS_PRICES\" \\"
        echo "  --gas-adjustment \"$GAS_ADJUSTMENT\" \\"
        echo "  --gas auto \\"
        echo "  --yes \\"
        echo "  --output json"
        return 0
    fi
    
    # 检查账户余额
    log_info "检查账户余额..."
    BALANCE=$(wasmd query bank balances "$ADMIN_ADDRESS" --node "$NODE" --output json | jq -r '.balances[] | select(.denom=="uluckee") | .amount // "0"')
    if [ "$BALANCE" = "0" ] || [ -z "$BALANCE" ]; then
        log_error "账户余额不足，请确保账户有足够的代币"
        exit 1
    fi
    log_info "账户余额: $BALANCE uluckee"
    
    # 上传合约
    log_info "正在上传合约..."
    UPLOAD_RESULT=$(wasmd tx wasm store target/wasm32-unknown-unknown/release/luckee_nft_optimized.wasm \
        --from "$ADMIN_ADDRESS" \
        --chain-id "$CHAIN_ID" \
        --node "$NODE" \
        --keyring-backend "$KEYRING_BACKEND" \
        --gas-prices "$GAS_PRICES" \
        --gas-adjustment "$GAS_ADJUSTMENT" \
        --gas auto \
        --yes \
        --output json)
    
    # 检查上传结果
    if [ $? -ne 0 ]; then
        log_error "上传合约失败"
        echo "$UPLOAD_RESULT"
        exit 1
    fi
    
    # 提取code_id
    CODE_ID=$(echo "$UPLOAD_RESULT" | jq -r '.logs[0].events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value')
    
    if [ -z "$CODE_ID" ] || [ "$CODE_ID" = "null" ]; then
        log_error "无法提取Code ID"
        echo "$UPLOAD_RESULT"
        exit 1
    fi
    
    log_info "合约上传成功，Code ID: $CODE_ID"
    
    # 保存到环境文件
    echo "CODE_ID=$CODE_ID" > .env
    if [ "$VERBOSE" = true ]; then
        echo "上传结果:"
        echo "$UPLOAD_RESULT" | jq '.'
    fi
}

# 实例化合约
instantiate_contract() {
    log_info "实例化合约..."
    
    if [ -z "$CODE_ID" ]; then
        log_error "CODE_ID 未设置"
        exit 1
    fi
    
    if [ -z "$MINTER_ADDRESS" ]; then
        log_error "请设置 MINTER_ADDRESS 环境变量"
        exit 1
    fi
    
    # 实例化消息
    INSTANTIATE_MSG=$(cat <<EOF
{
    "name": "Luckee NFT",
    "symbol": "LUCKEE",
    "minter": "$MINTER_ADDRESS",
    "base_uri": "https://luckee.io/metadata/"
}
EOF
)
    
    # 实例化合约
    INSTANTIATE_RESULT=$(wasmd tx wasm instantiate "$CODE_ID" "$INSTANTIATE_MSG" \
        --from "$ADMIN_ADDRESS" \
        --chain-id "$CHAIN_ID" \
        --node "$NODE" \
        --keyring-backend "$KEYRING_BACKEND" \
        --gas-prices "$GAS_PRICES" \
        --gas-adjustment "$GAS_ADJUSTMENT" \
        --gas auto \
        --label "luckee-nft-$CONTRACT_VERSION" \
        --admin "$ADMIN_ADDRESS" \
        --yes \
        --output json)
    
    # 提取合约地址
    CONTRACT_ADDRESS=$(echo "$INSTANTIATE_RESULT" | jq -r '.logs[0].events[] | select(.type=="instantiate") | .attributes[] | select(.key=="_contract_address") | .value')
    
    if [ -z "$CONTRACT_ADDRESS" ] || [ "$CONTRACT_ADDRESS" = "null" ]; then
        log_error "实例化合约失败"
        echo "$INSTANTIATE_RESULT"
        exit 1
    fi
    
    log_info "合约实例化成功，地址: $CONTRACT_ADDRESS"
    echo "CONTRACT_ADDRESS=$CONTRACT_ADDRESS" >> .env
}

# 配置合约
configure_contract() {
    log_info "配置合约..."
    
    if [ -z "$CONTRACT_ADDRESS" ]; then
        log_error "CONTRACT_ADDRESS 未设置"
        exit 1
    fi
    
    # 设置允许的铸造者（如果需要）
    if [ -n "$MINTER_ADDRESS" ]; then
        SET_MINTER_MSG=$(cat <<EOF
{
    "set_minter": {
        "minter": "$MINTER_ADDRESS",
        "allowed": true
    }
}
EOF
)
        
        wasmd tx wasm execute "$CONTRACT_ADDRESS" "$SET_MINTER_MSG" \
            --from "$ADMIN_ADDRESS" \
            --chain-id "$CHAIN_ID" \
            --node "$NODE" \
            --keyring-backend "$KEYRING_BACKEND" \
            --gas-prices "$GAS_PRICES" \
            --gas-adjustment "$GAS_ADJUSTMENT" \
            --gas auto \
            --yes \
            --output json
        
        log_info "设置铸造者权限完成"
    fi
}

# 验证部署
verify_deployment() {
    log_info "验证部署..."
    
    if [ -z "$CONTRACT_ADDRESS" ]; then
        log_error "CONTRACT_ADDRESS 未设置"
        exit 1
    fi
    
    # 查询合约信息
    CONTRACT_INFO=$(wasmd query wasm contract "$CONTRACT_ADDRESS" \
        --node "$NODE" \
        --output json)
    
    log_info "合约信息:"
    echo "$CONTRACT_INFO" | jq '.'
    
    # 查询合约配置
    CONFIG_QUERY='{"contract_info": {}}'
    CONFIG_RESULT=$(wasmd query wasm contract-state smart "$CONTRACT_ADDRESS" "$CONFIG_QUERY" \
        --node "$NODE" \
        --output json)
    
    log_info "合约配置:"
    echo "$CONFIG_RESULT" | jq '.data'
}

# 生成部署报告
generate_report() {
    log_info "生成部署报告..."
    
    REPORT_FILE="deployment_report_$(date +%Y%m%d_%H%M%S).md"
    
    cat > "$REPORT_FILE" <<EOF
# Luckee NFT 合约部署报告

## 部署信息
- **部署时间**: $(date)
- **链ID**: $CHAIN_ID
- **节点**: $NODE
- **合约版本**: $CONTRACT_VERSION

## 合约信息
- **Code ID**: $CODE_ID
- **合约地址**: $CONTRACT_ADDRESS
- **管理员**: $ADMIN_ADDRESS
- **铸造者**: $MINTER_ADDRESS

## 部署步骤
1. ✅ 检查依赖
2. ✅ 构建合约
3. ✅ 上传合约
4. ✅ 实例化合约
5. ✅ 配置合约
6. ✅ 验证部署

## 后续步骤
1. 在盲盒合约中设置NFT合约地址
2. 测试NFT铸造功能
3. 测试合成功能
4. 配置前端集成

## 环境变量
\`\`\`bash
export CODE_ID=$CODE_ID
export CONTRACT_ADDRESS=$CONTRACT_ADDRESS
export ADMIN_ADDRESS=$ADMIN_ADDRESS
export MINTER_ADDRESS=$MINTER_ADDRESS
\`\`\`
EOF
    
    log_info "部署报告已生成: $REPORT_FILE"
}

# 主函数
main() {
    log_info "开始部署 Luckee NFT 合约..."
    
    # 显示配置信息
    if [ "$VERBOSE" = true ]; then
        log_info "部署配置:"
        echo "  链ID: $CHAIN_ID"
        echo "  节点: $NODE"
        echo "  管理员: $ADMIN_ADDRESS"
        echo "  铸造者: $MINTER_ADDRESS"
        echo "  Gas价格: $GAS_PRICES"
        echo "  Gas调整: $GAS_ADJUSTMENT"
        echo "  密钥环: $KEYRING_BACKEND"
        echo ""
    fi
    
    # 安全提示
    if [ "$CHAIN_ID" != "luckee-testnet" ] && [ "$DRY_RUN" = false ]; then
        log_warn "⚠️  警告: 您正在部署到非测试网环境"
        log_warn "请确保:"
        log_warn "1. 使用多签钱包作为管理员地址"
        log_warn "2. 在冷钱包中执行关键操作"
        log_warn "3. 不要在CI/CD中保存私钥"
        echo ""
        read -p "确认继续部署? (yes/no): " confirm
        if [ "$confirm" != "yes" ]; then
            log_info "部署已取消"
            exit 0
        fi
    fi
    
    check_dependencies
    build_contract
    upload_contract
    instantiate_contract
    configure_contract
    verify_deployment
    generate_report
    
    log_info "部署完成！"
    log_info "合约地址: $CONTRACT_ADDRESS"
    log_info "Code ID: $CODE_ID"
    
    # 显示后续步骤
    if [ "$DRY_RUN" = false ]; then
        echo ""
        log_info "后续步骤:"
        log_info "1. 在盲盒合约中设置NFT合约地址: $CONTRACT_ADDRESS"
        log_info "2. 测试NFT铸造功能"
        log_info "3. 测试合成功能"
        log_info "4. 配置前端集成"
        echo ""
        log_info "环境变量已保存到 .env 文件"
    fi
}

# 帮助信息
show_help() {
    cat <<EOF
Luckee NFT 合约部署脚本

用法: $0 [选项]

选项:
    -h, --help              显示帮助信息
    -c, --chain-id          链ID (默认: luckee-testnet)
    -n, --node              节点地址 (默认: https://rpc.luckee-testnet.com:443)
    -a, --admin             管理员地址 (必需)
    -m, --minter            铸造者地址 (必需)
    --dry-run               干运行模式，仅显示将要执行的命令
    --verbose               详细输出模式
    --skip-build            跳过构建步骤
    --skip-upload           跳过上传步骤
    --skip-instantiate      跳过实例化步骤
    --skip-configure        跳过配置步骤

环境变量:
    ADMIN_ADDRESS           管理员地址
    MINTER_ADDRESS          铸造者地址
    CHAIN_ID               链ID
    NODE                    节点地址
    GAS_PRICES              Gas价格
    GAS_ADJUSTMENT          Gas调整系数
    KEYRING_BACKEND         密钥环后端

示例:
    # 完整部署
    $0 --admin luckee1... --minter luckee1...
    
    # 干运行模式
    $0 --admin luckee1... --minter luckee1... --dry-run
    
    # 跳过构建，仅上传和实例化
    $0 --admin luckee1... --minter luckee1... --skip-build
    
    # 使用环境变量
    ADMIN_ADDRESS=luckee1... MINTER_ADDRESS=luckee1... $0
    
    # 详细输出模式
    $0 --admin luckee1... --minter luckee1... --verbose

安全提示:
    - 在主网部署时，请使用多签钱包作为管理员地址
    - 不要在CI/CD中保存私钥
    - 建议在冷钱包中执行关键操作
EOF
}

# 解析命令行参数
DRY_RUN=false
VERBOSE=false
SKIP_BUILD=false
SKIP_UPLOAD=false
SKIP_INSTANTIATE=false
SKIP_CONFIGURE=false

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
        --dry-run)
            DRY_RUN=true
            log_info "启用干运行模式"
            shift
            ;;
        --verbose)
            VERBOSE=true
            log_info "启用详细输出模式"
            shift
            ;;
        --skip-build)
            SKIP_BUILD=true
            log_info "跳过构建步骤"
            shift
            ;;
        --skip-upload)
            SKIP_UPLOAD=true
            log_info "跳过上传步骤"
            shift
            ;;
        --skip-instantiate)
            SKIP_INSTANTIATE=true
            log_info "跳过实例化步骤"
            shift
            ;;
        --skip-configure)
            SKIP_CONFIGURE=true
            log_info "跳过配置步骤"
            shift
            ;;
        *)
            log_error "未知参数: $1"
            show_help
            exit 1
            ;;
    esac
done

# 运行主函数
main
