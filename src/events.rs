//! 事件生成模块
//! 
//! 此模块包含所有标准 CW721 事件和 Luckee 扩展事件的生成函数
//! 用于在区块链上发出可索引的事件，方便外部应用监听和查询

use cosmwasm_std::{Addr, Event};

// ========== 事件属性常量 ==========

/// 事件属性键常量，统一管理所有事件属性名称
/// 便于下游 indexer 解析和索引
pub mod event_attributes {
    /// 操作类型属性键
    pub const ACTION: &str = "action";
    /// NFT ID 属性键
    pub const TOKEN_ID: &str = "token_id";
    /// 所有者地址属性键
    pub const OWNER: &str = "owner";
    /// 接收者地址属性键
    pub const RECIPIENT: &str = "recipient";
    /// 发送者地址属性键
    pub const FROM: &str = "from";
    /// 目标地址属性键
    pub const TO: &str = "to";
    /// 被批准者地址属性键
    pub const SPENDER: &str = "spender";
    /// 操作员地址属性键
    pub const OPERATOR: &str = "operator";
    /// NFT 类型属性键
    pub const KIND: &str = "kind";
    /// 系列 ID 属性键
    pub const SERIES_ID: &str = "series_id";
    /// 系列序号属性键
    pub const SERIAL: &str = "serial";
    /// 总供应量属性键
    pub const TOTAL_SUPPLY: &str = "total_supply";
    /// 输入数量属性键
    pub const INPUTS_COUNT: &str = "inputs_count";
    /// 输出 NFT ID 属性键
    pub const OUTPUT_TOKEN_ID: &str = "output_token_id";
    /// 目标类型属性键
    pub const TARGET: &str = "target";
}

/// 操作类型常量，统一管理所有操作类型
pub mod action_types {
    /// 铸造操作
    pub const MINT: &str = "mint";
    /// 销毁操作
    pub const BURN: &str = "burn";
    /// 转移操作
    pub const TRANSFER: &str = "transfer";
    /// 批准操作
    pub const APPROVE: &str = "approve";
    /// 撤销批准操作
    pub const REVOKE: &str = "revoke";
    /// 批准所有操作
    pub const APPROVE_ALL: &str = "approve_all";
    /// 撤销所有批准操作
    pub const REVOKE_ALL: &str = "revoke_all";
    /// 合成操作
    pub const SYNTHESIZE: &str = "synthesize";
    /// 批量铸造操作
    pub const BATCH_MINT: &str = "batch_mint";
}

// ========== 标准 CW721 事件 ==========

/// 生成铸造事件
/// 
/// 当新的 NFT 被铸造时发出此事件
/// 
/// # 参数
/// - `token_id`: NFT ID
/// - `owner`: 所有者地址
/// - `kind`: NFT 类型
/// 
/// # 返回值
/// - `Event`: 铸造事件
pub fn emit_mint_event(token_id: u64, owner: &str, kind: &str) -> Event {
    Event::new("wasm")
        .add_attribute(event_attributes::ACTION, action_types::MINT)
        .add_attribute(event_attributes::TOKEN_ID, token_id.to_string())
        .add_attribute(event_attributes::OWNER, owner)
        .add_attribute(event_attributes::KIND, kind)
}

/// 生成销毁事件
/// 
/// 当 NFT 被销毁时发出此事件
/// 
/// # 参数
/// - `token_id`: NFT ID
/// - `owner`: 所有者地址
/// 
/// # 返回值
/// - `Event`: 销毁事件
pub fn emit_burn_event(token_id: u64, owner: &Addr) -> Event {
    Event::new("wasm")
        .add_attribute(event_attributes::ACTION, action_types::BURN)
        .add_attribute(event_attributes::TOKEN_ID, token_id.to_string())
        .add_attribute(event_attributes::OWNER, owner.to_string())
}

/// 生成转移事件
/// 
/// 当 NFT 所有权发生转移时发出此事件
/// 
/// # 参数
/// - `token_id`: NFT ID
/// - `from`: 原所有者地址
/// - `to`: 新所有者地址
/// 
/// # 返回值
/// - `Event`: 转移事件
pub fn emit_transfer_event(token_id: u64, from: &Addr, to: &Addr) -> Event {
    Event::new("wasm")
        .add_attribute(event_attributes::ACTION, action_types::TRANSFER)
        .add_attribute(event_attributes::TOKEN_ID, token_id.to_string())
        .add_attribute(event_attributes::FROM, from.to_string())
        .add_attribute(event_attributes::TO, to.to_string())
}

/// 生成批准事件
/// 
/// 当 NFT 被批准给特定地址时发出此事件
/// 
/// # 参数
/// - `token_id`: NFT ID
/// - `owner`: 所有者地址
/// - `spender`: 被批准者地址
/// 
/// # 返回值
/// - `Event`: 批准事件
pub fn emit_approval_event(token_id: u64, owner: &Addr, spender: &Addr) -> Event {
    Event::new("wasm")
        .add_attribute(event_attributes::ACTION, action_types::APPROVE)
        .add_attribute(event_attributes::TOKEN_ID, token_id.to_string())
        .add_attribute(event_attributes::OWNER, owner.to_string())
        .add_attribute(event_attributes::SPENDER, spender.to_string())
}

/// 生成撤销批准事件
/// 
/// 当 NFT 的批准被撤销时发出此事件
/// 
/// # 参数
/// - `token_id`: NFT ID
/// - `owner`: 所有者地址
/// - `spender`: 被撤销批准的地址
/// 
/// # 返回值
/// - `Event`: 撤销批准事件
pub fn emit_revoke_event(token_id: u64, owner: &Addr, spender: &Addr) -> Event {
    Event::new("wasm")
        .add_attribute(event_attributes::ACTION, action_types::REVOKE)
        .add_attribute(event_attributes::TOKEN_ID, token_id.to_string())
        .add_attribute(event_attributes::OWNER, owner.to_string())
        .add_attribute(event_attributes::SPENDER, spender.to_string())
}

/// 生成批准所有事件
/// 
/// 当操作员被批准管理所有 NFT 时发出此事件
/// 
/// # 参数
/// - `owner`: 所有者地址
/// - `operator`: 操作员地址
/// 
/// # 返回值
/// - `Event`: 批准所有事件
pub fn emit_approve_all_event(owner: &Addr, operator: &Addr) -> Event {
    Event::new("wasm")
        .add_attribute(event_attributes::ACTION, action_types::APPROVE_ALL)
        .add_attribute(event_attributes::OWNER, owner.to_string())
        .add_attribute(event_attributes::OPERATOR, operator.to_string())
}

/// 生成撤销所有批准事件
/// 
/// 当操作员对所有 NFT 的批准被撤销时发出此事件
/// 
/// # 参数
/// - `owner`: 所有者地址
/// - `operator`: 操作员地址
/// 
/// # 返回值
/// - `Event`: 撤销所有批准事件
pub fn emit_revoke_all_event(owner: &Addr, operator: &Addr) -> Event {
    Event::new("wasm")
        .add_attribute(event_attributes::ACTION, action_types::REVOKE_ALL)
        .add_attribute(event_attributes::OWNER, owner.to_string())
        .add_attribute(event_attributes::OPERATOR, operator.to_string())
}

// ========== Luckee 扩展事件 ==========

/// 生成合成事件
/// 
/// 当 NFT 合成操作完成时发出此事件
/// 
/// # 参数
/// - `output_token_id`: 输出 NFT ID
/// - `target`: 目标 NFT 类型
/// - `inputs_count`: 输入 NFT 数量
/// - `user`: 执行合成的用户地址
/// 
/// # 返回值
/// - `Event`: 合成事件
pub fn emit_synthesize_event(output_token_id: u64, target: &str, inputs_count: usize, user: &Addr) -> Event {
    Event::new("wasm")
        .add_attribute(event_attributes::ACTION, action_types::SYNTHESIZE)
        .add_attribute(event_attributes::OUTPUT_TOKEN_ID, output_token_id.to_string())
        .add_attribute(event_attributes::TARGET, target)
        .add_attribute(event_attributes::INPUTS_COUNT, inputs_count.to_string())
        .add_attribute(event_attributes::OWNER, user.to_string())
}

/// 生成批量铸造事件
/// 
/// 当批量铸造操作完成时发出此事件
/// 
/// # 参数
/// - `count`: 铸造的 NFT 数量
/// - `minter`: 铸造者地址
/// 
/// # 返回值
/// - `Event`: 批量铸造事件
pub fn emit_batch_mint_event(count: usize, minter: &Addr) -> Event {
    Event::new("wasm")
        .add_attribute(event_attributes::ACTION, action_types::BATCH_MINT)
        .add_attribute(event_attributes::INPUTS_COUNT, count.to_string())
        .add_attribute(event_attributes::OWNER, minter.to_string())
}
