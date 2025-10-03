//! 消息定义模块
//! 
//! 此模块定义了合约的所有消息类型，包括：
//! - 初始化消息 (InstantiateMsg)
//! - 执行消息 (ExecuteMsg)
//! - 查询消息 (QueryMsg)
//! - 各种响应类型

use cosmwasm_schema::{cw_serde, QueryResponses};
use crate::types::{NftKind, NftMeta, Recipe, RecipeInput};
use crate::state::Expiration;

// ========== 初始化消息 ==========

/// 合约初始化消息
/// 
/// 在合约部署时传递的配置参数
#[cw_serde]
pub struct InstantiateMsg {
    /// 合约名称
    pub name: String,
    /// 合约符号
    pub symbol: String,
    /// 铸造者地址（盲盒合约地址）
    pub minter: String,
    /// 基础 URI（可选）
    pub base_uri: Option<String>,
}

// ========== 执行消息 ==========

/// 合约执行消息
/// 
/// 定义所有可以执行的合约操作
#[cw_serde]
pub enum ExecuteMsg {
    // ========== 标准 CW721 接口 ==========
    /// 转移 NFT 所有权
    TransferNft { recipient: String, token_id: u64 },
    /// 批准特定地址操作特定 NFT
    Approve { spender: String, token_id: u64, expires: Option<Expiration> },
    /// 撤销特定地址对特定 NFT 的批准
    Revoke { spender: String, token_id: u64 },
    /// 批准操作员管理所有 NFT
    ApproveAll { operator: String, expires: Option<Expiration> },
    /// 撤销操作员对所有 NFT 的管理权限
    RevokeAll { operator: String },
    
    // ========== Luckee 扩展接口 ==========
    /// 铸造新的 NFT
    Mint { 
        token_id: u64, 
        owner: String, 
        extension: NftMeta 
    },
    /// 销毁 NFT
    Burn { token_id: u64 },
    
    // ========== 管理员接口 ==========
    /// 更新铸造者地址
    UpdateMinter { new_minter: String },
    /// 更新基础 URI
    UpdateBaseUri { base_uri: String },
    
    // ========== 合成相关接口 ==========
    /// 设置合成配方
    SetRecipe { target: NftKind, recipe: Recipe },
    /// 删除合成配方
    RemoveRecipe { target: NftKind },
    /// 执行合成操作
    Synthesize { inputs: Vec<u64>, target: NftKind },
    
    // ========== 批量操作接口 ==========
    /// 批量铸造 NFT
    BatchMint { mints: Vec<BatchMintItem> },
    /// 设置铸造者权限
    SetMinter { minter: String, allowed: bool },
    
    
    // ========== 访问控制和紧急机制 ==========
    /// 暂停合约
    Pause {},
    /// 恢复合约
    Unpause {},
    /// 紧急提取资金
    EmergencyWithdraw { amount: Vec<cosmwasm_std::Coin> },
}

// ========== 查询消息 ==========

/// 合约查询消息
/// 
/// 定义所有可以查询的合约信息
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // ========== 标准 CW721 查询 ==========
    /// 查询 NFT 所有者信息
    #[returns(cw721::OwnerOfResponse)]
    OwnerOf { token_id: u64, include_expired: Option<bool> },
    
    /// 查询 NFT 详细信息
    #[returns(cw721::NftInfoResponse<NftMeta>)]
    NftInfo { token_id: u64 },
    
    /// 查询 NFT 批准信息
    #[returns(cw721::ApprovalsResponse)]
    Approvals { token_id: u64, include_expired: Option<bool> },
    
    /// 查询操作员批准状态
    #[returns(cw721::OperatorResponse)]
    IsApprovedForAll { owner: String, operator: String },
    
    /// 查询 NFT URI 信息
    #[returns(cw721::NftInfoResponse<NftMeta>)]
    TokenUri { token_id: u64 },
    
    /// 查询所有 NFT 列表
    #[returns(cw721::TokensResponse)]
    AllTokens { start_after: Option<u64>, limit: Option<u32> },
    
    /// 查询用户拥有的 NFT 列表
    #[returns(cw721::TokensResponse)]
    Tokens { owner: String, start_after: Option<u64>, limit: Option<u32> },
    
    // ========== Luckee 扩展查询 ==========
    /// 查询 NFT 扩展元数据
    #[returns(TokenMetaResponse)]
    TokenMeta { token_id: u64 },
    
    /// 按类型查询 NFT 列表
    #[returns(TokensByKindResponse)]
    TokensByKind { kind: NftKind, start_after: Option<u64>, limit: Option<u32> },
    
    /// 按系列查询 NFT 列表
    #[returns(TokensBySeriesResponse)]
    TokensBySeries { series_id: String, start_after: Option<u64>, limit: Option<u32> },
    
    /// 按组查询 NFT 列表
    #[returns(TokensByGroupResponse)]
    TokensByGroup { group_id: String, start_after: Option<u64>, limit: Option<u32> },
    
    /// 查询 Luckee 合约信息
    #[returns(LuckeeContractInfoResponse)]
    LuckeeContractInfo {},
    
    // ========== 合成相关查询 ==========
    /// 查询合成配方
    #[returns(RecipeResponse)]
    Recipe { target: NftKind },
    
    /// 查询所有合成配方
    #[returns(AllRecipesResponse)]
    AllRecipes { start_after: Option<NftKind>, limit: Option<u32> },
    
    /// 预览合成操作结果
    #[returns(SynthesisPreviewResponse)]
    SynthesisPreview { inputs: Vec<u64>, target: NftKind },
    
    // ========== CW721 集成查询 ==========
    /// 查询外部 CW721 合约地址
    #[returns(NftContractResponse)]
    GetNftContract {},
    
    // ========== 标准 CW721 合约信息查询 ==========
    /// 查询标准 CW721 合约信息
    #[returns(cw721::ContractInfoResponse)]
    ContractInfo {},
}

// ========== 查询响应类型 ==========

/// NFT 元数据查询响应
#[cw_serde]
pub struct TokenMetaResponse {
    /// NFT 元数据
    pub meta: NftMeta,
}

/// 按类型查询 NFT 响应
#[cw_serde]
pub struct TokensByKindResponse {
    /// NFT ID 列表
    pub tokens: Vec<u64>,
}

/// 按系列查询 NFT 响应
#[cw_serde]
pub struct TokensBySeriesResponse {
    /// NFT ID 列表
    pub tokens: Vec<u64>,
}

/// 按组查询 NFT 响应
#[cw_serde]
pub struct TokensByGroupResponse {
    /// NFT ID 列表
    pub tokens: Vec<u64>,
}

/// Luckee 合约信息查询响应
#[cw_serde]
pub struct LuckeeContractInfoResponse {
    /// 合约名称
    pub name: String,
    /// 合约符号
    pub symbol: String,
    /// 铸造者地址
    pub minter: String,
    /// 基础 URI
    pub base_uri: Option<String>,
    /// 总供应量
    pub total_supply: u64,
}

/// 批量铸造项目
#[cw_serde]
pub struct BatchMintItem {
    /// NFT ID
    pub token_id: u64,
    /// 所有者地址
    pub owner: String,
    /// NFT 元数据
    pub extension: NftMeta,
}

/// 合成配方查询响应
#[cw_serde]
pub struct RecipeResponse {
    /// 合成配方（如果存在）
    pub recipe: Option<Recipe>,
}

/// 所有合成配方查询响应
#[cw_serde]
pub struct AllRecipesResponse {
    /// 配方列表（目标类型，配方）
    pub recipes: Vec<(NftKind, Recipe)>,
}

/// 合成预览查询响应
#[cw_serde]
pub struct SynthesisPreviewResponse {
    /// 是否可以合成
    pub can_synthesize: bool,
    /// 需要的输入
    pub required_inputs: Vec<RecipeInput>,
    /// 输出值
    pub output_value: u32,
    /// 合成成本（可选）
    pub cost: Option<cosmwasm_std::Coin>,
}

/// 外部 NFT 合约查询响应
#[cw_serde]
pub struct NftContractResponse {
    /// 外部合约地址（如果设置）
    pub contract_addr: Option<String>,
}
