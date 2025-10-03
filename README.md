# Luckee NFT 合约

基于 CosmWasm 的扩展 CW721 NFT 合约，支持盲盒奖励分发、合成机制和系列管理。

## 功能特性

- **9种主题NFT**：四叶草、流萤、赤色锦鲤、三愿神灯、命运纺锤、悟道者、紫薇帝星、轮盘之主、造化元灵
- **盲盒奖励分发**：根据抽奖结果自动分发对应NFT
- **合成机制**：支持NFT合成升级（轮盘之主、造化元灵）
- **系列管理**：支持多创建人、多系列的软合并和硬合并
- **物理实物绑定**：NFT可绑定实物SKU
- **兑换价值系统**：基于合成成本的动态价值计算

## 项目结构

```
luckee_nft/
├── contracts/
│   ├── nft/              # 主NFT合约
│   └── factory/          # NFT工厂合约（可选）
├── packages/
│   ├── types/            # 共享类型定义
│   └── utils/            # 工具函数
├── docs/                 # 设计文档
├── scripts/              # 部署脚本
└── tests/                # 测试用例
```

## 快速开始

### 1. 构建合约

```bash
# 安装依赖
cargo build

# 优化构建
RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown

# 压缩wasm文件
wasm-opt -Os target/wasm32-unknown-unknown/release/luckee_nft.wasm -o target/wasm32-unknown-unknown/release/luckee_nft_optimized.wasm
```

### 2. 运行测试

```bash
cargo test
```

### 3. 部署到测试网

```bash
# 设置环境变量
export ADMIN_ADDRESS="your_admin_address"
export MINTER_ADDRESS="blind_box_contract_address"

# 运行部署脚本
./scripts/deploy.sh
```

## NFT 类型与稀有度

### 九种主题NFT

| NFT类型 | 稀有度等级 | 兑换价值 | 获取方式 |
|---------|-----------|----------|----------|
| 四叶草 (Clover) | 0 - Common | 1 | 盲盒未中奖 |
| 流萤 (Firefly) | 1 - Uncommon | 2 | 盲盒末等奖 |
| 赤色锦鲤 (Koi) | 2 - Rare | 4 | Tiny规模头奖 |
| 三愿神灯 (Lamp) | 3 - Epic | 20 | Small规模头奖 |
| 命运纺锤 (Spindle) | 4 - Legendary | 200 | Medium规模头奖 |
| 悟道者 (Sage) | 5 - Mythic | 2000 | Large规模头奖 |
| 紫薇帝星 (Polaris) | 6 - Divine | 20000 | Huge规模头奖 |
| 轮盘之主 (Roulette) | 7 - Transcendent | 200000 | 合成获得 |
| 造化元灵 (Genesis) | 8 - Genesis | 2000000 | 合成获得 |

### 合成配方

| 目标NFT | 所需材料 | 数量 |
|---------|----------|------|
| 流萤 | 四叶草 | 2个 |
| 赤色锦鲤 | 流萤 | 2个 |
| 三愿神灯 | 赤色锦鲤 | 5个 |
| 命运纺锤 | 三愿神灯 | 10个 |
| 悟道者 | 命运纺锤 | 10个 |
| 紫薇帝星 | 悟道者 | 10个 |
| 轮盘之主 | 紫薇帝星 | 10个 |
| 造化元灵 | 轮盘之主 | 10个 |

## 合约接口

### 执行消息

```rust
pub enum ExecuteMsg {
    // 标准CW721接口
    TransferNft { recipient: String, token_id: u64 },
    Approve { spender: String, token_id: u64 },
    Revoke { spender: String, token_id: u64 },
    ApproveAll { operator: String },
    RevokeAll { operator: String },
    
    // 扩展接口
    Mint { token_id: u64, owner: String, extension: NftMeta },
    Burn { token_id: u64 },
    
    // 合成相关接口
    SetRecipe { target: NftKind, recipe: Recipe },
    RemoveRecipe { target: NftKind },
    Synthesize { inputs: Vec<u64>, target: NftKind },
    
    // 批量操作接口
    BatchMint { mints: Vec<BatchMintItem> },
    SetMinter { minter: String, allowed: bool },
    
    // 管理员接口
    UpdateMinter { new_minter: String },
    UpdateBaseUri { base_uri: String },
}
```

### 查询消息

```rust
pub enum QueryMsg {
    // 标准CW721查询
    OwnerOf { token_id: u64 },
    NftInfo { token_id: u64 },
    Approval { token_id: u64 },
    IsApprovedForAll { owner: String, operator: String },
    TokenUri { token_id: u64 },
    AllTokens { start_after: Option<u64>, limit: Option<u32> },
    Tokens { owner: String, start_after: Option<u64>, limit: Option<u32> },
    
    // 扩展查询
    TokenMeta { token_id: u64 },
    TokensByKind { kind: NftKind, start_after: Option<u64>, limit: Option<u32> },
    TokensBySeries { series_id: String, start_after: Option<u64>, limit: Option<u32> },
    TokensByGroup { group_id: String, start_after: Option<u64>, limit: Option<u32> },
    ContractInfo {},
    
    // 合成相关查询
    Recipe { target: NftKind },
    AllRecipes { start_after: Option<NftKind>, limit: Option<u32> },
    SynthesisPreview { inputs: Vec<u64>, target: NftKind },
}
```

## 与现有盲盒合约集成

### 1. 在盲盒合约中设置NFT合约地址

```rust
// 在盲盒合约中添加
ExecuteMsg::SetNftContract { addr: String }
```

### 2. 盲盒奖励分发

```rust
// 在盲盒合约的Finalize函数中添加
let mint_msg = ExecuteMsg::Mint {
    token_id: next_token_id,
    owner: recipient.to_string(),
    extension: NftMeta {
        kind: nft_kind,
        scale_origin: scale,
        physical_sku: None,
        crafted_from: None,
        series_id: format!("blind_box_{}", blind_box_id),
        collection_group_id: None,
        serial_in_series: serial,
    },
};

// 调用NFT合约
wasmd::tx::wasm::execute(
    deps.as_mut(),
    nft_contract_addr,
    &mint_msg,
    vec![],
)?;
```

## 部署配置

### 环境变量

```bash
# 必需
export ADMIN_ADDRESS="luckee1..."      # 管理员地址
export MINTER_ADDRESS="luckee1..."      # 铸造者地址（盲盒合约地址）

# 可选
export CHAIN_ID="luckee-testnet"        # 链ID
export NODE="https://rpc.luckee-testnet.com:443"  # 节点地址
export GAS_PRICES="0.025uluckee"        # Gas价格
```

### 部署步骤

1. **构建合约**
   ```bash
   cargo build --release --target wasm32-unknown-unknown
   ```

2. **上传合约**
   ```bash
   wasmd tx wasm store target/wasm32-unknown-unknown/release/luckee_nft_optimized.wasm \
     --from $ADMIN_ADDRESS --chain-id $CHAIN_ID --gas auto --yes
   ```

3. **实例化合约**
   ```bash
   wasmd tx wasm instantiate $CODE_ID '{"name":"Luckee NFT","symbol":"LUCKEE","minter":"'$MINTER_ADDRESS'","base_uri":"https://luckee.io/metadata/"}' \
     --from $ADMIN_ADDRESS --chain-id $CHAIN_ID --label "luckee-nft" --admin $ADMIN_ADDRESS --yes
   ```

4. **配置权限**
   ```bash
   wasmd tx wasm execute $CONTRACT_ADDRESS '{"set_minter":{"minter":"'$MINTER_ADDRESS'","allowed":true}}' \
     --from $ADMIN_ADDRESS --chain-id $CHAIN_ID --yes
   ```

## 测试

### 运行测试

```bash
# 单元测试
cargo test

# 集成测试
cargo test --test integration

# 特定测试
cargo test test_synthesis
```

### 测试覆盖

- ✅ NFT铸造功能
- ✅ NFT转移和授权
- ✅ 合成机制
- ✅ 批量操作
- ✅ 权限控制
- ✅ 错误处理

## 安全考虑

### 访问控制
- NFT合约的minter权限仅授予盲盒合约和合成合约
- 合成操作需要用户明确授权
- 管理员操作需要owner权限

### 防重入攻击
- 合成操作先更新状态，再执行burn/mint
- 使用状态机控制操作顺序
- 设置操作上限防止DoS

### 输入验证
- 严格验证NFT所有权
- 检查合成配方的合法性
- 验证兑换价值的合理性

## 文档

- [合约设计文档](docs/合约设计.md)
- [API接口文档](docs/接口文档.md)
- [部署指南](docs/部署指南.md)

## 许可证

MIT License