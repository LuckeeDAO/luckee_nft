//! 合约错误定义模块
//! 
//! 此模块定义了合约中可能出现的所有错误类型
//! 使用 thiserror 库提供标准化的错误处理

use cosmwasm_std::StdError;
use thiserror::Error;

/// 合约错误枚举
/// 
/// 定义了合约执行过程中可能出现的各种错误情况
#[derive(Error, Debug)]
pub enum ContractError {
    /// 标准 CosmWasm 错误
    #[error("{0}")]
    Std(#[from] StdError),

    /// 未授权操作
    #[error("Unauthorized")]
    Unauthorized {},

    /// NFT 不存在
    #[error("Token not found")]
    TokenNotFound {},

    /// NFT 已存在
    #[error("Token already exists")]
    TokenAlreadyExists {},

    /// 无效的 NFT 类型
    #[error("Invalid NFT kind")]
    InvalidNftKind {},

    /// 无效的合成配方
    #[error("Invalid synthesis recipe")]
    InvalidSynthesisRecipe {},

    /// 无效的交换值
    #[error("Invalid exchange value: {value}")]
    InvalidExchangeValue { value: u8 },

    /// 合成值不足
    #[error("Insufficient value for combination: required {required}, got {got}")]
    InsufficientValue { required: u32, got: u32 },

    /// 无效的组合配方
    #[error("Invalid combination recipe")]
    InvalidRecipe {},

    /// NFT 不属于发送者
    #[error("Token not owned by sender")]
    NotOwned {},

    /// NFT 未被批准
    #[error("Token not approved")]
    NotApproved {},

    /// 系列不存在
    #[error("Series not found")]
    SeriesNotFound {},

    /// 组不存在
    #[error("Group not found")]
    GroupNotFound {},

    /// 合并操作失败
    #[error("Merge operation failed")]
    MergeFailed {},

    /// 数值溢出错误
    #[error("Overflow error")]
    Overflow {},

    /// 合约已暂停
    #[error("Contract is paused")]
    ContractPaused {},

    /// 无效的状态转换
    #[error("Invalid state transition")]
    InvalidStateTransition {},

    /// 组合中的 NFT 数量过多
    #[error("Too many tokens in combination: {count}")]
    TooManyTokens { count: usize },

    /// 组合中存在循环依赖
    #[error("Circular dependency in combination")]
    CircularDependency {},
    
    /// 合成不被允许
    #[error("Synthesis not allowed")]
    SynthesisNotAllowed {},
    
    /// 输入 NFT 数量不足
    #[error("Insufficient input tokens")]
    InsufficientInputTokens {},
    
    /// 配方不存在
    #[error("Recipe not found")]
    RecipeNotFound {},
    
    /// 铸造者未授权
    #[error("Minter not authorized")]
    MinterNotAuthorized {},

    /// 外部铸造失败
    #[error("External mint failed")]
    ExternalMintFailed {},

    /// 外部销毁失败
    #[error("External burn failed")]
    ExternalBurnFailed {},

    /// 未知的回复 ID
    #[error("Unknown reply ID: {id}")]
    UnknownReplyId { id: u64 },

    /// 合成上下文不存在
    #[error("Synthesis context not found")]
    SynthesisContextNotFound {},

    /// 无效的回复数据
    #[error("Invalid reply data")]
    InvalidReplyData {},

    /// 需要外部合约但未设置
    #[error("External contract required but not set")]
    ExternalContractRequired {},

    /// 合成输入数量过多
    #[error("Too many inputs for synthesis: {count}")]
    TooManyInputs { count: usize },
}
