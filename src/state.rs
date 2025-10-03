//! 合约状态存储模块
//! 
//! 此模块定义了合约的所有存储结构和状态管理
//! 包括配置、NFT 数据、索引和各种辅助状态

#[cfg(feature = "cosmwasm")]
use cosmwasm_std::Addr;
#[cfg(feature = "cosmwasm")]
use cw_storage_plus::{Item, Map};

#[cfg(feature = "std")]
use schemars::JsonSchema;

use serde::{Deserialize, Serialize};
use crate::types::{NftMeta, Recipe};

// ========== 数据结构定义 ==========

/// 合约配置结构
/// 
/// 存储合约的基本配置信息
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct Config {
    /// 合约名称
    pub name: String,
    /// 合约符号
    pub symbol: String,
    /// 铸造者地址
    pub minter: Addr,
    /// 基础 URI（可选）
    pub base_uri: Option<String>,
    /// 合约所有者地址
    pub owner: Addr,
}

// ========== 存储项定义 ==========

/// 合约配置存储
#[cfg(feature = "cosmwasm")]
pub const CONFIG: Item<Config> = Item::new("config");

/// NFT ID 到元数据的映射
#[cfg(feature = "cosmwasm")]
pub const TOKEN_META: Map<u64, NftMeta> = Map::new("token_meta");

/// 系列 ID 到下一个序号的映射
#[cfg(feature = "cosmwasm")]
pub const SERIES_NEXT_SERIAL: Map<String, u64> = Map::new("series_next_serial");

/// 总供应量存储
#[cfg(feature = "cosmwasm")]
pub const TOTAL_SUPPLY: Item<u64> = Item::new("total_supply");

/// 下一个Token ID计数器
/// 用于确保Token ID的唯一性，即使有Token被销毁
#[cfg(feature = "cosmwasm")]
pub const NEXT_TOKEN_ID: Item<u64> = Item::new("next_token_id");

// ========== 扩展存储项定义 ==========

/// 规模到头奖 NFT 的映射
/// 键: 规模字符串，值: 头奖 NFT 类型
#[cfg(feature = "cosmwasm")]
pub const FIRST_PRIZE_BY_SCALE: Map<String, String> = Map::new("first_prize");

/// NFT 兑换价值表
/// 键: NFT 类型字符串，值: 兑换价值
#[cfg(feature = "cosmwasm")]
pub const EXCHANGE_VALUE: Map<String, u8> = Map::new("exchange_value");

/// 合成配方映射
/// 键: NFT 类型字符串，值: 合成配方
#[cfg(feature = "cosmwasm")]
pub const RECIPES: Map<String, Recipe> = Map::new("recipes");

/// 系列到集合组的映射
/// 键: 系列 ID，值: 集合组 ID
#[cfg(feature = "cosmwasm")]
pub const SERIES_TO_GROUP: Map<String, String> = Map::new("series_group");

/// 物理 SKU 映射
/// 键: SKU ID，值: 物理商品信息
#[cfg(feature = "cosmwasm")]
pub const SKU_TABLE: Map<String, String> = Map::new("sku_table");

/// 合成历史记录映射
/// 键: (用户地址, 时间戳)，值: 合成记录
#[cfg(feature = "cosmwasm")]
pub const SYNTHESIS_HISTORY: Map<(Addr, u64), SynthesisRecord> = Map::new("synthesis_history");

// ========== 数据结构定义 ==========

/// 合成记录结构
/// 
/// 记录一次合成操作的详细信息
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct SynthesisRecord {
    /// 执行合成的用户地址
    pub user: Addr,
    /// 输入的 NFT ID 列表
    pub inputs: Vec<u64>,
    /// 输出的 NFT ID
    pub output: u64,
    /// 合成时间戳
    pub timestamp: u64,
}

// ========== 权限和状态存储 ==========

/// 允许的铸造者列表
/// 键: 铸造者地址，值: 是否允许铸造
pub const ALLOWED_MINTERS: Map<Addr, bool> = Map::new("allowed_minters");


/// 合约暂停状态
/// true: 合约已暂停，false: 合约正常运行
#[cfg(feature = "cosmwasm")]
pub const CONTRACT_PAUSED: Item<bool> = Item::new("contract_paused");

// ========== 版本和待处理操作存储 ==========

/// 存储版本信息
/// 用于合约升级时的版本管理
#[cfg(feature = "cosmwasm")]
pub const STORAGE_VERSION: Item<String> = Item::new("storage_version");


/// 本地 NFT 所有权映射（用于 metadata-only 模式）
/// 键: NFT ID，值: 所有者地址
#[cfg(feature = "cosmwasm")]
pub const LOCAL_OWNERSHIP: Map<u64, Addr> = Map::new("local_ownership");

// ========== CW721 标准存储结构 ==========

/// NFT ID 到所有者的映射
/// 键: NFT ID，值: 所有者地址
#[cfg(feature = "cosmwasm")]
pub const TOKEN_OWNERSHIP: Map<u64, Addr> = Map::new("token_owners");

/// NFT ID 到批准信息的映射
/// 键: NFT ID，值: 批准信息列表
#[cfg(feature = "cosmwasm")]
pub const TOKEN_APPROVALS: Map<u64, Vec<crate::state::Approval>> = Map::new("token_approvals");

/// 所有者到操作员批准的映射
/// 键: (所有者地址, 操作员地址)，值: 批准过期时间
#[cfg(feature = "cosmwasm")]
pub const OPERATOR_APPROVALS: Map<(Addr, Addr), Expiration> = Map::new("operator_approvals");

/// 所有者拥有的所有 NFT ID 列表
/// 键: 所有者地址，值: 拥有的 NFT ID 列表
#[cfg(feature = "cosmwasm")]
pub const TOKENS_BY_OWNER: Map<Addr, Vec<u64>> = Map::new("tokens_by_owner");

/// 所有 NFT ID 的枚举
/// 键: NFT ID，值: 空值（仅用于枚举）
#[cfg(feature = "cosmwasm")]
pub const ALL_TOKENS: Map<u64, ()> = Map::new("all_tokens");

/// 合约信息存储
/// 包含合约的基本信息（名称、符号等）
#[cfg(feature = "cosmwasm")]
pub const CONTRACT_INFO: Item<ContractInfo> = Item::new("contract_info");

// ========== CW721 标准数据结构 ==========

/// 批准信息结构
/// 
/// 存储 NFT 的批准信息，包括被批准的操作员和过期时间
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct Approval {
    /// 被批准的操作员地址
    pub spender: Addr,
    /// 批准过期时间（可选）
    pub expires: Option<Expiration>,
}

/// 过期时间结构
/// 
/// 定义批准或授权的过期条件
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct Expiration {
    /// 按区块高度过期
    pub at_height: Option<u64>,
    /// 按时间戳过期
    pub at_time: Option<u64>,
}

/// 合约信息结构
/// 
/// 存储合约的基本标识信息
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct ContractInfo {
    /// 合约名称
    pub name: String,
    /// 合约符号
    pub symbol: String,
}

// ========== 实现方法 ==========

impl Expiration {
    /// 检查是否已过期
    /// 
    /// 根据当前环境信息检查批准是否已过期
    /// 
    /// # 参数
    /// - `env`: 环境信息，包含当前区块高度和时间
    /// 
    /// # 返回值
    /// - `bool`: true 表示已过期，false 表示未过期
    #[cfg(feature = "cosmwasm")]
    pub fn is_expired(&self, env: &cosmwasm_std::Env) -> bool {
        // 检查区块高度过期
        if let Some(at_height) = self.at_height {
            if env.block.height >= at_height {
                return true;
            }
        }
        // 检查时间戳过期
        if let Some(at_time) = self.at_time {
            if env.block.time.seconds() >= at_time {
                return true;
            }
        }
        false
    }
}
