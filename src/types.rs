//! 类型定义模块
//! 
//! 此模块定义了合约中使用的所有核心数据类型，包括：
//! - NFT 类型枚举 (NftKind)
//! - 盲盒规模枚举 (Scale)
//! - NFT 元数据结构 (NftMeta)
//! - 合成配方相关结构
//! - 各种请求和响应结构

use cosmwasm_schema::cw_serde;

// ========== NFT 类型定义 ==========

/// 九种主题 NFT 类型枚举
/// 
/// 定义了合约支持的所有 NFT 类型，每种类型都有不同的稀有度和价值
#[cw_serde]
pub enum NftKind {
    /// 四叶草 - 未中奖，最基础的 NFT 类型
    Clover,
    /// 流萤 - 末等奖，通过合成获得
    Firefly,
    /// 赤色锦鲤 - Tiny 规模头奖
    CrimsonKoi,
    /// 三愿神灯 - Small 规模头奖
    MagicalLamp,
    /// 命运纺锤 - Medium 规模头奖
    FatesSpindle,
    /// 悟道者 - Large 规模头奖
    Sage,
    /// 紫薇帝星 - Huge 规模头奖
    Polaris,
    /// 轮盘之主 - 通过合成获得的高级 NFT
    WheelOfDestiny,
    /// 造化元灵 - 通过合成获得的顶级 NFT
    Genesis,
}

// ========== NftKind 实现方法 ==========

impl NftKind {
    /// 获取 NFT 的稀有度等级
    /// 
    /// 返回 0-8 的数字，数字越大表示越稀有
    /// 
    /// # 返回值
    /// - `u8`: 稀有度等级
    pub fn rarity_level(&self) -> u8 {
        match self {
            NftKind::Clover => 0,           // 最普通
            NftKind::Firefly => 1,          // 普通
            NftKind::CrimsonKoi => 2,       // 稀有
            NftKind::MagicalLamp => 3,      // 史诗
            NftKind::FatesSpindle => 4,     // 传说
            NftKind::Sage => 5,             // 神话
            NftKind::Polaris => 6,          // 神圣
            NftKind::WheelOfDestiny => 7,   // 超越
            NftKind::Genesis => 8,          // 创世
        }
    }
    
    /// 获取 NFT 的稀有度名称
    /// 
    /// 返回对应的英文稀有度名称
    /// 
    /// # 返回值
    /// - `&'static str`: 稀有度名称
    pub fn rarity_name(&self) -> &'static str {
        match self {
            NftKind::Clover => "Common",
            NftKind::Firefly => "Uncommon", 
            NftKind::CrimsonKoi => "Rare",
            NftKind::MagicalLamp => "Epic",
            NftKind::FatesSpindle => "Legendary",
            NftKind::Sage => "Mythic",
            NftKind::Polaris => "Divine",
            NftKind::WheelOfDestiny => "Transcendent",
            NftKind::Genesis => "Genesis",
        }
    }
    
    /// 获取 NFT 的兑换价值
    /// 
    /// 返回以四叶草为基准单位的兑换价值
    /// 
    /// # 返回值
    /// - `u32`: 兑换价值（四叶草数量）
    pub fn exchange_value(&self) -> u32 {
        match self {
            NftKind::Clover => 1,           // 四叶草 - 基础价值单位
            NftKind::Firefly => 2,          // 流萤 - 2个四叶草合成
            NftKind::CrimsonKoi => 4,       // 赤色锦鲤 - 4个四叶草合成
            NftKind::MagicalLamp => 20,     // 三愿神灯 - 20个四叶草合成
            NftKind::FatesSpindle => 200,   // 命运纺锤 - 200个四叶草合成
            NftKind::Sage => 2000,          // 悟道者 - 2000个四叶草合成
            NftKind::Polaris => 20000,      // 紫薇帝星 - 20000个四叶草合成
            NftKind::WheelOfDestiny => 200000, // 轮盘之主 - 200000个四叶草合成
            NftKind::Genesis => 2000000,    // 造化元灵 - 2000000个四叶草合成
        }
    }
}

impl NftKind {
    /// 将 NftKind 转换为字符串键
    /// 
    /// 用于存储和序列化时的键值转换
    /// 
    /// # 返回值
    /// - `String`: 对应的字符串键
    pub fn to_key(&self) -> String {
        format!("{:?}", self)
    }
    
    /// 从字符串键转换为 NftKind
    /// 
    /// 用于从存储中读取时的键值转换
    /// 
    /// # 参数
    /// - `key`: 字符串键
    /// 
    /// # 返回值
    /// - `Result<Self, cosmwasm_std::StdError>`: 转换结果
    pub fn from_key(key: &str) -> Result<Self, cosmwasm_std::StdError> {
        match key {
            "Clover" => Ok(NftKind::Clover),
            "Firefly" => Ok(NftKind::Firefly),
            "CrimsonKoi" => Ok(NftKind::CrimsonKoi),
            "MagicalLamp" => Ok(NftKind::MagicalLamp),
            "FatesSpindle" => Ok(NftKind::FatesSpindle),
            "Sage" => Ok(NftKind::Sage),
            "Polaris" => Ok(NftKind::Polaris),
            "WheelOfDestiny" => Ok(NftKind::WheelOfDestiny),
            "Genesis" => Ok(NftKind::Genesis),
            _ => Err(cosmwasm_std::StdError::generic_err(format!("Unknown NftKind: {}", key)))
        }
    }
}

// ========== 盲盒规模定义 ==========

/// 盲盒规模枚举
/// 
/// 定义了盲盒的不同规模，每种规模对应不同的头奖 NFT
#[cw_serde]
pub enum Scale {
    /// 微型规模
    Tiny,
    /// 小型规模
    Small,
    /// 中型规模
    Medium,
    /// 大型规模
    Large,
    /// 巨型规模
    Huge,
}

// ========== Scale 实现方法 ==========

impl Scale {
    /// 获取规模对应的头奖 NFT 类型
    /// 
    /// 根据盲盒规模返回对应的头奖 NFT 类型
    /// 
    /// # 返回值
    /// - `NftKind`: 对应的头奖 NFT 类型
    pub fn first_prize_nft(&self) -> NftKind {
        match self {
            Scale::Tiny => NftKind::CrimsonKoi,
            Scale::Small => NftKind::MagicalLamp,
            Scale::Medium => NftKind::FatesSpindle,
            Scale::Large => NftKind::Sage,
            Scale::Huge => NftKind::Polaris,
        }
    }
}

// ========== NFT 元数据结构 ==========

/// NFT 扩展元数据结构
/// 
/// 存储 NFT 的详细元数据信息，包括类型、来源、合成历史等
#[cw_serde]
pub struct NftMeta {
    /// NFT 种类（包含稀有度信息）
    pub kind: NftKind,
    /// 来源规模（盲盒规模）
    pub scale_origin: Scale,
    /// 物理实物 SKU（可选）
    pub physical_sku: Option<String>,
    /// 合成来源 TokenId 列表（可选）
    pub crafted_from: Option<Vec<u64>>,
    /// 系列 ID
    pub series_id: String,
    /// 集合组 ID（用于合并，可选）
    pub collection_group_id: Option<String>,
    /// 系列内序号
    pub serial_in_series: u64,
}

// ========== 合成相关结构 ==========

/// 合成配方结构
/// 
/// 定义如何将多个 NFT 合成为一个新的 NFT
#[cw_serde]
pub struct Recipe {
    /// 输入 NFT 列表
    pub inputs: Vec<RecipeInput>,
    /// 输出的 NFT 类型
    pub output: NftKind,
    /// 合成费用（可选）
    pub cost: Option<cosmwasm_std::Coin>,
}

/// 合成配方输入结构
/// 
/// 定义合成配方中单个输入项的要求
#[cw_serde]
pub struct RecipeInput {
    /// 需要的 NFT 类型
    pub nft_kind: NftKind,
    /// 需要的数量
    pub count: u32,
}

// ========== 请求结构 ==========

/// 合成请求结构
/// 
/// 用户提交的合成操作请求
#[cw_serde]
pub struct CombineRequest {
    /// 输入的 TokenId 列表
    pub inputs: Vec<u64>,
    /// 目标 NFT 类型
    pub target: NftKind,
}

/// 系列合并请求结构
/// 
/// 用于将多个系列合并为一个系列的请求
#[cw_serde]
pub struct MergeSeriesRequest {
    /// 源系列 ID 列表
    pub from_series: Vec<String>,
    /// 目标系列 ID
    pub target_series: String,
    /// 是否保留元数据
    pub preserve_metadata: bool,
}
